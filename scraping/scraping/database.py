from os import getenv
from datetime import datetime

from sqlalchemy import create_engine, Column
from sqlalchemy.ext.declarative import AbstractConcreteBase, declarative_base
from sqlalchemy.orm import sessionmaker, relationship, configure_mappers
from sqlalchemy.dialects.postgresql import ARRAY

from sqlalchemy import (
    Integer, String,DateTime, Boolean, ForeignKey,
)

from scrapy.utils.project import get_project_settings

Base = declarative_base()
postgres_sync_engine = create_engine(
    url=getenv('SA_DATABASE_URL'), 
    echo=False,
)
Session = sessionmaker(postgres_sync_engine, expire_on_commit=False)


class BaseModel(AbstractConcreteBase, Base):
    id = Column(Integer, primary_key=True)


class LangModel(BaseModel):
    __tablename__ = "languages"

    code = Column(String(2), nullable=False)
    iso_name = Column(String(128), nullable=False)
    native_name = Column(String(256), nullable=False)


class TranslationModel(BaseModel):
    __tablename__ = "bible_translations"

    lang_id = Column(
        Integer,
        ForeignKey('languages.id', ondelete='CASCADE'), 
        nullable=False,
    )
    lang = relationship("LangModel", backref="translations")
    name = Column(String(128))
    description = Column(String(256))
    aliases = Column(ARRAY(String))


class BookModel(BaseModel):
    __tablename__ = 'bible_books'

    tr_id = Column(
        Integer,
        ForeignKey('bible_translations.id', ondelete='CASCADE'), 
        nullable=False,
    )
    tr = relationship('TranslationModel', backref='books')
    no = Column(Integer, nullable=False)
    key = Column(String(16))
    name = Column(String(256))
    aliases = Column(ARRAY(String), nullable=False)
    is_new_testament = Column(Boolean)
    created_at = Column(DateTime, default=datetime.now, nullable=False)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now, nullable=False)


class BibleContentModel(BaseModel):
    __tablename__ = 'bible_content'
    
    book_id = Column(
        Integer,
        ForeignKey('bible_books.id', ondelete='CASCADE'), 
        nullable=False,
        index=True,
    )
    book = relationship('BookModel', backref='content')
    chapter = Column(Integer, nullable=False)
    vers = Column(Integer, nullable=False)
    text = Column(String, nullable=False)
    created_at = Column(DateTime, default=datetime.now, nullable=False)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now, nullable=False)

