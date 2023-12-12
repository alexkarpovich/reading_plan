import re
from pathlib import Path

import scrapy

from scraping.items import (
    BibleonlineTranslation, 
    BibleonlineBook,
    BibleonlineChapter,
)


class BibleonlineSpider(scrapy.Spider):
    name = "bibleonline"
    allowed_domains = ["www.bibleonline.ru"]
    start_urls = ["https://www.bibleonline.ru/bible/"]

    def parse(self, response):
        print("procesing:"+response.url)
        for tr in response.css('div#all a.list-group-item'):
            url = tr.xpath('@href').get()
            lang_code = tr.xpath('img/@class').get()
            name = tr.xpath('.//span[1]/text()').get()
            desc = tr.xpath('.//span[2]/text()').get()
            alias = Path(url).name

            yield BibleonlineTranslation(
                alias=alias,
                lang_code=lang_code[-2:],
                name=name,
                desc=desc,
            )

            yield scrapy.Request(
                response.urljoin(url),
                callback=self.parse_bible,
                meta={'tr_alias': alias},
            )
                

    def parse_bible(self, response):
        no = 1

        for testament_el in response.xpath('//div[@id="biblelist"]/div/div'):
            book_elements = testament_el.xpath('//div[contains(@class,"biblebooks")]//a[@class="list-group-item"]')
            books_count = len(book_elements)

            for book_el in book_elements:
                book_url = book_el.xpath('@href').get()
                name = book_el.xpath('text()').get()
                key = book_el.xpath('span/@book').get().strip()

                yield BibleonlineBook(
                    tr_alias=response.meta['tr_alias'],
                    no=no,
                    key=key,
                    name=name.strip(),
                    is_new_testament=books_count == 27,
                )

                no += 1

                chapter_first_url = f'{re.sub("(-dsc|-toc)", "", book_url)[:-1]}-1/'

                yield scrapy.Request(
                    response.urljoin(chapter_first_url),
                    callback=self.parse_chapter,
                    meta={
                        'tr_alias': response.meta['tr_alias'], 
                        'book_key': key,
                    },
                )
            
    def parse_chapter(self, response):
        chapter_no = int(response.xpath('//div[contains(@class, "top-pagination")]//li[contains(@class,"active pgn")]/@value').get() or 1)
        verses = {}

        for vers_el in response.xpath('//div[contains(@class,"bible-text")]//span[contains(@class, "v-init")]'):
            no = int(vers_el.xpath('@vers').get())
            content = ''.join(vers_el.xpath('.//text()').extract())

            verses[no] = content.strip()

        yield BibleonlineChapter(
            tr_alias=response.meta['tr_alias'],
            book_key=response.meta['book_key'],
            no=chapter_no,
            verses=verses,
        )

        next_el = response.xpath('//ul[@class="pager"]//li[@class="next"]/a')
        next_url = next_el.xpath('@href').get()
        next_text = next_el.xpath('text()').get()

        if next_text.strip().isnumeric():
            yield scrapy.Request(
                response.urljoin(next_url),
                callback=self.parse_chapter,
                meta={
                    'tr_alias': response.meta['tr_alias'], 
                    'book_key': response.meta['book_key'],
                },
            )