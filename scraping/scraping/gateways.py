from sqlalchemy import select, insert
from sqlalchemy.exc import SQLAlchemyError

from scraping.database import (
    Session, 
    LangModel,
    TranslationModel,
    BookModel,
    BibleContentModel,
)


class TranslationGateway:
    def __init__(self, session: Session) -> None:
        self.session = session

    def add(
            self, 
            name: str, 
            description: str, 
            lang_code: str,
            aliases: list[str],
    ) -> TranslationModel:
        query = (
            select(LangModel.id)
            .where(
                LangModel.code == lang_code,
            )
        )

        lang_id = self.session.scalars(query).one()
        
        tr = TranslationModel(
            lang_id=lang_id,
            name=name,
            description=description,
            aliases=aliases,
        )

        try:
            self.session.add(tr)
            self.session.commit()
        except Exception as e:
            self.session.rollback()
            raise e

        return tr
    
    def get_by_alias(
            self,
            alias: str,
    ) -> int:
        query = (
            select(TranslationModel.id)
            .where(
                TranslationModel.aliases.contains([alias])
            )
        )

        return self.session.scalars(query).one()

class BibleGateway:
    def __init__(self, session: Session) -> None:
        self.session = session

    def add_book(
            self,
            tr_id: int,
            no: int,
            key: str,
            name: str,
            aliases: list[str],
            is_new_testament: bool,
    ) -> BookModel:
        book = BookModel(
            tr_id=tr_id, 
            no=no, 
            key=key,
            name=name, 
            aliases=aliases, 
            is_new_testament=is_new_testament,
        )

        try:
            self.session.add(book)
            self.session.commit()
        except Exception as e:
            self.session.rollback()
            raise e

        return book
    
    def get_book_by_key(
            self,
            tr_alias: str,
            book_key: str,
    ) -> int:
        query = (
            select(BookModel.id)
            .where(
                BookModel.key == book_key,
                BookModel.tr_id == TranslationModel.id,
                TranslationModel.aliases.contains([tr_alias])
            )
        )

        return self.session.scalars(query).one()
    
    def get_content_by_chapter(
            self,
            book_id: int,
            chapter: int,
    ) -> int:
        query = (
            select(BibleContentModel.id)
            .where(
                BibleContentModel.book_id == book_id,
                BibleContentModel.chapter == chapter,
            )
        )

        return self.session.scalars(query).one()
    
    def bulk_content_insert(
            self,
            items: list,
    ) -> None:  
        stmt = insert(BibleContentModel).values(items)

        try:
            self.session.execute(stmt)
            self.session.commit()
        except Exception as e:
            self.session.rollback()
            raise e
