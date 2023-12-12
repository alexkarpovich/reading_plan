# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


class BibleonlineTranslation(scrapy.Item):
    alias = scrapy.Field()
    lang_code = scrapy.Field()
    name = scrapy.Field()
    desc = scrapy.Field()


class BibleonlineBook(scrapy.Item):
    tr_alias = scrapy.Field()
    no = scrapy.Field()
    key = scrapy.Field()
    name = scrapy.Field()
    is_new_testament = scrapy.Field()


class BibleonlineChapter(scrapy.Item):
    tr_alias = scrapy.Field()
    book_key = scrapy.Field()
    no = scrapy.Field()
    verses = scrapy.Field()