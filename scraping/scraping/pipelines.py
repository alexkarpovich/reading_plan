# Define your item pipelines here
#
# Don't forget to add your pipeline to the ITEM_PIPELINES setting
# See: https://docs.scrapy.org/en/latest/topics/item-pipeline.html


# useful for handling different item types with a single interface
from itemadapter import ItemAdapter

from scraping.database import (
    Session,
)
from scraping.gateways import (
    TranslationGateway,
    BibleGateway,
)
from scraping.items import (
    BibleonlineTranslation,
    BibleonlineBook,
    BibleonlineChapter,
)


class ScrapingPipeline:
    def process_item(self, item, spider):
        
        return item
    

class SavePipeline:
    def __init__(self) -> None:
        self.session = Session()
        self.tr_gateway = TranslationGateway(session=self.session)
        self.bible_gateway = BibleGateway(session=self.session)

    def process_translation(self, item: BibleonlineTranslation):
        print('Start processing translation', item)

        try:
            self.tr_gateway.get_by_alias(item['alias'])
        except:
            self.tr_gateway.add(
                name=item['name'], 
                description=item['desc'], 
                lang_code=item['lang_code'],
                aliases=[item['alias']],
            )
            

    def process_book(self, item: BibleonlineBook):
        print('Start processing book', item)

        try:
            self.bible_gateway.get_book_by_key(
                tr_alias=item['tr_alias'],
                book_key=item['key'],
            )
        except:
            tr_id = self.tr_gateway.get_by_alias(alias=item['tr_alias'])

            self.bible_gateway.add_book(
                tr_id=tr_id,
                no=item['no'],
                name=item['name'],
                key=item['key'],
                aliases=[item['key']],
                is_new_testament=item['is_new_testament'],
            )
            
    def process_chapter(self, item: BibleonlineChapter):
        print('Start processing chapter', item)

        book_id = self.bible_gateway.get_book_by_key(
            tr_alias=item['tr_alias'],
            book_key=item['book_key'],
        )

        try:
            self.bible_gateway.get_content_by_chapter(
                book_id=book_id,
                chapter=item['no'],
            )
        except:
            self.bible_gateway.bulk_content_insert(
                items=[
                    {
                        'book_id': book_id,
                        'chapter': item['no'],
                        'vers': no, 
                        'text': content,
                    } for no, content in item['verses'].items()
                ],
            )

    def process_item(self, item, spider):
        {
            BibleonlineTranslation: self.process_translation,
            BibleonlineBook: self.process_book,
            BibleonlineChapter: self.process_chapter,
        }[type(item)](item)

        return item
