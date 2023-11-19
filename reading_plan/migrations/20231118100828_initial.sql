-- Add migration script here
CREATE TABLE languages (
	code VARCHAR (3) UNIQUE NOT NULL,
	name VARCHAR (64) NOT NULL
);

CREATE TABLE bible_books (
	id serial PRIMARY KEY,
	name VARCHAR (64) UNIQUE NOT NULL,
	is_new_testament BOOLEAN NOT NULL
);

CREATE TABLE bible_book_tr (
	id serial PRIMARY KEY,
	book_id INT NOT NULL,
    lang_code VARCHAR(3) NOT NULL,
    name VARCHAR(64) NOT NULL,

    CONSTRAINT fk_book
      FOREIGN KEY(book_id) 
	  REFERENCES bible_books(id),

	CONSTRAINT fk_lang
      FOREIGN KEY(lang_code) 
	  REFERENCES languages(code)
);

CREATE TABLE bible_refs (
	id serial PRIMARY KEY,
	book_id INT NOT NULL,
    chapter INT NOT NULL,
    verse INT NOT NULL,

    CONSTRAINT fk_book
      FOREIGN KEY(book_id) 
	  REFERENCES bible_books(id)
);

CREATE TABLE bible_verses (
	id serial PRIMARY KEY,
	ref_id INT NOT NULL,
    lang_code INT NOT NULL,
    content TEXT NOT NULL,
    
    CONSTRAINT fk_book_ref
      FOREIGN KEY(ref_id) 
	  REFERENCES bible_refs(id),

    CONSTRAINT fk_lang
      FOREIGN KEY(lang_code) 
	  REFERENCES languages(code)
);