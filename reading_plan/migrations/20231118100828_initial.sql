-- Add migration script here
CREATE TABLE IF NOT EXISTS languages (
	id serial PRIMARY KEY,
	code CHAR (2) UNIQUE NOT NULL,
	iso_name VARCHAR (32) NOT NULL,
	native_name VARCHAR(32) NOT NULL
);

CREATE INDEX idx_languages_code
	ON languages(code);

CREATE TABLE IF NOT EXISTS bible_translations (
	id serial PRIMARY KEY,
	lang_id INT NOT NULL
		REFERENCES languages (id) ON DELETE CASCADE,
	name VARCHAR (128) NOT NULL,
	description VARCHAR (256) NOT NULL,
	aliases VARCHAR(32)[] NOT NULL
);

CREATE TABLE IF NOT EXISTS bible_books (
	id serial PRIMARY KEY,
	tr_id INT NOT NULL 
		REFERENCES bible_translations (id) ON DELETE CASCADE,
	
	no INT NOT NULL,
	key VARCHAR (16) NOT NULL,
	name VARCHAR (64) NOT NULL,
	aliases VARCHAR(32)[] NOT NULL,
	is_new_testament BOOLEAN NOT NULL,
	created_at TIMESTAMP NOT NULL,
	updated_at TIMESTAMP NOT NULL,

	UNIQUE (tr_id, name),
	UNIQUE (tr_id, key)
);

CREATE INDEX idx_bible_books_name
	ON bible_books(name);

CREATE TABLE IF NOT EXISTS bible_content (
	id serial PRIMARY KEY,
	book_id INT NOT NULL 
		REFERENCES bible_books (id) ON DELETE CASCADE,
	chapter INT NOT NULL,
	vers INT NOT NULL,
	text TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL,
	updated_at TIMESTAMP NOT NULL,

	UNIQUE (book_id, chapter, vers)
);

CREATE INDEX idx_bible_content_book_id
	ON bible_content(book_id);

CREATE VIEW content AS
  	SELECT bc.*, bb.tr_id as tr_id, bb.no as book_no, 
		concat_ws(
			'.', 
			LPAD(bb.no::text, 3, '0'),
			LPAD(bc.chapter::text, 3, '0'),
			LPAD(bc.vers::text, 3, '0')
		) as key
	FROM bible_content bc
	LEFT JOIN bible_books bb ON bb.id = bc.book_id
	ORDER BY key;

INSERT INTO languages (id, code, iso_name, native_name) VALUES 
(1, 'aa', 'Afar', 'Afaraf'),
(2, 'ab', 'Abkhaz', 'аҧсуа бызшәа'),
(3, 'ae', 'Avestan', 'avesta'),
(4, 'af', 'Afrikaans', 'Afrikaans'),
(5, 'ak', 'Akan', 'Akan'),
(6, 'am', 'Amharic', 'አማርኛ'),
(7, 'an', 'Aragonese', 'aragonés'),
(8, 'ar', 'Arabic', 'اللغة العربية'),
(9, 'as', 'Assamese', 'অসমীয়া'),
(10, 'av', 'Avaric', 'авар мацӀ'),
(11, 'ay', 'Aymara', 'aymar aru'),
(12, 'az', 'Azerbaijani', 'azərbaycan dili'),
(13, 'ba', 'Bashkir', 'башҡорт теле'),
(14, 'be', 'Belarusian', 'беларуская мова'),
(15, 'bg', 'Bulgarian', 'български език'),
(16, 'bh', 'Bihari', 'भोजपुरी'),
(17, 'bi', 'Bislama', 'Bislama'),
(18, 'bm', 'Bambara', 'bamanankan'),
(19, 'bn', 'Bengali', 'বাংলা'),
(20, 'bo', 'Tibetan', 'བོད་ཡིག'),
(21, 'br', 'Breton', 'brezhoneg'),
(22, 'bs', 'Bosnian', 'bosanski jezik'),
(23, 'ca', 'Catalan', 'Català'),
(24, 'ce', 'Chechen', 'нохчийн мотт'),
(25, 'ch', 'Chamorro', 'Chamoru'),
(26, 'co', 'Corsican', 'corsu'),
(27, 'cr', 'Cree', 'ᓀᐦᐃᔭᐍᐏᐣ'),
(28, 'cs', 'Czech', 'čeština'),
(29, 'cu', 'Old Church Slavonic', 'ѩзыкъ словѣньскъ'),
(30, 'cv', 'Chuvash', 'чӑваш чӗлхи'),
(31, 'cy', 'Welsh', 'Cymraeg'),
(32, 'da', 'Danish', 'dansk'),
(33, 'de', 'German', 'Deutsch'),
(34, 'dv', 'Divehi', 'Dhivehi'),
(35, 'dz', 'Dzongkha', 'རྫོང་ཁ'),
(36, 'ee', 'Ewe', 'Eʋegbe'),
(37, 'el', 'Greek', 'Ελληνικά'),
(38, 'en', 'English', 'English'),
(39, 'eo', 'Esperanto', 'Esperanto'),
(40, 'es', 'Spanish', 'Español'),
(41, 'et', 'Estonian', 'eesti'),
(42, 'eu', 'Basque', 'euskara'),
(43, 'fa', 'Persian', 'فارسی'),
(44, 'ff', 'Fula', 'Fulfulde'),
(45, 'fi', 'Finnish', 'suomi'),
(46, 'fj', 'Fijian', 'Vakaviti'),
(47, 'fo', 'Faroese', 'føroyskt'),
(48, 'fr', 'French', 'Français'),
(49, 'fy', 'Western Frisian', 'Frysk'),
(50, 'ga', 'Irish', 'Gaeilge'),
(51, 'gd', 'Scottish Gaelic', 'Gàidhlig'),
(52, 'gl', 'Galician', 'galego'),
(53, 'gu', 'Gujarati', 'ગુજરાતી'),
(54, 'gv', 'Manx', 'Gaelg'),
(55, 'ha', 'Hausa', 'هَوُسَ'),
(56, 'he', 'Hebrew', 'עברית'),
(57, 'hi', 'Hindi', 'हिन्दी'),
(58, 'ho', 'Hiri Motu', 'Hiri Motu'),
(59, 'hr', 'Croatian', 'Hrvatski'),
(60, 'ht', 'Haitian', 'Kreyòl ayisyen'),
(61, 'hu', 'Hungarian', 'magyar'),
(62, 'hy', 'Armenian', 'Հայերեն'),
(63, 'hz', 'Herero', 'Otjiherero'),
(64, 'ia', 'Interlingua', 'Interlingua'),
(65, 'id', 'Indonesian', 'Bahasa Indonesia'),
(66, 'ie', 'Interlingue', 'Interlingue'),
(67, 'ig', 'Igbo', 'Asụsụ Igbo'),
(68, 'ii', 'Nuosu', 'ꆈꌠ꒿ Nuosuhxop'),
(69, 'ik', 'Inupiaq', 'Iñupiaq'),
(70, 'io', 'Ido', 'Ido'),
(71, 'is', 'Icelandic', 'Íslenska'),
(72, 'it', 'Italian', 'Italiano'),
(73, 'iu', 'Inuktitut', 'ᐃᓄᒃᑎᑐᑦ'),
(74, 'ja', 'Japanese', '日本語'),
(75, 'jv', 'Javanese', 'basa Jawa'),
(76, 'ka', 'Georgian', 'ქართული'),
(77, 'kg', 'Kongo', 'Kikongo'),
(78, 'ki', 'Kikuyu', 'Gĩkũyũ'),
(79, 'kj', 'Kwanyama', 'Kuanyama'),
(80, 'kk', 'Kazakh', 'қазақ тілі'),
(81, 'kl', 'Kalaallisut', 'kalaallisut'),
(82, 'km', 'Khmer', 'ខេមរភាសា'),
(83, 'kn', 'Kannada', 'ಕನ್ನಡ'),
(84, 'ko', 'Korean', '한국어'),
(85, 'kr', 'Kanuri', 'Kanuri'),
(86, 'ks', 'Kashmiri', 'कश्मीरी'),
(87, 'ku', 'Kurdish', 'Kurdî'),
(88, 'kv', 'Komi', 'коми кыв'),
(89, 'kw', 'Cornish', 'Kernewek'),
(90, 'ky', 'Kyrgyz', 'Кыргызча'),
(91, 'la', 'Latin', 'latine'),
(92, 'lb', 'Luxembourgish', 'Lëtzebuergesch'),
(93, 'lg', 'Ganda', 'Luganda'),
(94, 'li', 'Limburgish', 'Limburgs'),
(95, 'ln', 'Lingala', 'Lingála'),
(96, 'lo', 'Lao', 'ພາສາ'),
(97, 'lt', 'Lithuanian', 'lietuvių kalba'),
(98, 'lu', 'Luba-Katanga', 'Tshiluba'),
(99, 'lv', 'Latvian', 'latviešu valoda'),
(100, 'mg', 'Malagasy', 'fiteny malagasy'),
(101, 'mh', 'Marshallese', 'Kajin M̧ajeļ'),
(102, 'mi', 'Māori', 'te reo Māori'),
(103, 'mk', 'Macedonian', 'македонски јазик'),
(104, 'ml', 'Malayalam', 'മലയാളം'),
(105, 'mn', 'Mongolian', 'Монгол хэл'),
(106, 'mr', 'Marathi', 'मराठी'),
(107, 'ms', 'Malay', 'Bahasa Malaysia'),
(108, 'mt', 'Maltese', 'Malti'),
(109, 'my', 'Burmese', 'ဗမာစာ'),
(110, 'na', 'Nauru', 'Ekakairũ Naoero'),
(111, 'nb', 'Norwegian Bokmål', 'Norsk bokmål'),
(112, 'nd', 'Northern Ndebele', 'isiNdebele'),
(113, 'ne', 'Nepali', 'नेपाली'),
(114, 'ng', 'Ndonga', 'Owambo'),
(115, 'nl', 'Dutch', 'Nederlands'),
(116, 'nn', 'Norwegian Nynorsk', 'Norsk nynorsk'),
(117, 'no', 'Norwegian', 'Norsk'),
(118, 'nr', 'Southern Ndebele', 'isiNdebele'),
(119, 'nv', 'Navajo', 'Diné bizaad'),
(120, 'ny', 'Chichewa', 'chiCheŵa'),
(121, 'oc', 'Occitan', 'occitan'),
(122, 'oj', 'Ojibwe', 'ᐊᓂᔑᓈᐯᒧᐎᓐ'),
(123, 'om', 'Oromo', 'Afaan Oromoo'),
(124, 'or', 'Oriya', 'ଓଡ଼ିଆ'),
(125, 'os', 'Ossetian', 'ирон æвзаг'),
(126, 'pa', 'Panjabi', 'ਪੰਜਾਬੀ'),
(127, 'pi', 'Pāli', 'पाऴि'),
(128, 'pl', 'Polish', 'Polski'),
(129, 'ps', 'Pashto', 'پښتو'),
(130, 'pt', 'Portuguese', 'Português'),
(131, 'qu', 'Quechua', 'Runa Simi'),
(132, 'rm', 'Romansh', 'rumantsch grischun'),
(133, 'rn', 'Kirundi', 'Ikirundi'),
(134, 'ro', 'Romanian', 'Română'),
(135, 'ru', 'Russian', 'Русский'),
(136, 'rw', 'Kinyarwanda', 'Ikinyarwanda'),
(137, 'sa', 'Sanskrit', 'संस्कृतम्'),
(138, 'sc', 'Sardinian', 'sardu'),
(139, 'sd', 'Sindhi', 'सिन्धी'),
(140, 'se', 'Northern Sami', 'Davvisámegiella'),
(141, 'sg', 'Sango', 'yângâ tî sängö'),
(142, 'si', 'Sinhala', 'සිංහල'),
(143, 'sk', 'Slovak', 'slovenčina'),
(144, 'sl', 'Slovenian', 'slovenščina'),
(145, 'sn', 'Shona', 'chiShona'),
(146, 'so', 'Somali', 'Soomaaliga'),
(147, 'sq', 'Albanian', 'Shqip'),
(148, 'sr', 'Serbian', 'српски језик'),
(149, 'ss', 'Swati', 'SiSwati'),
(150, 'st', 'Southern Sotho', 'Sesotho'),
(151, 'su', 'Sundanese', 'Basa Sunda'),
(152, 'sv', 'Swedish', 'Svenska'),
(153, 'sw', 'Swahili', 'Kiswahili'),
(154, 'ta', 'Tamil', 'தமிழ்'),
(155, 'te', 'Telugu', 'తెలుగు'),
(156, 'tg', 'Tajik', 'тоҷикӣ'),
(157, 'th', 'Thai', 'ไทย'),
(158, 'ti', 'Tigrinya', 'ትግርኛ'),
(159, 'tk', 'Turkmen', 'Türkmen'),
(160, 'tl', 'Tagalog', 'Wikang Tagalog'),
(161, 'tn', 'Tswana', 'Setswana'),
(162, 'to', 'Tonga', 'faka Tonga'),
(163, 'tr', 'Turkish', 'Türkçe'),
(164, 'ts', 'Tsonga', 'Xitsonga'),
(165, 'tt', 'Tatar', 'татар теле'),
(166, 'tw', 'Twi', 'Twi'),
(167, 'ty', 'Tahitian', 'Reo Tahiti'),
(168, 'ug', 'Uyghur', 'ئۇيغۇرچە‎'),
(169, 'uk', 'Ukrainian', 'Українська'),
(170, 'ur', 'Urdu', 'اردو'),
(171, 'uz', 'Uzbek', 'Ўзбек'),
(172, 've', 'Venda', 'Tshivenḓa'),
(173, 'vi', 'Vietnamese', 'Tiếng Việt'),
(174, 'vo', 'Volapük', 'Volapük'),
(175, 'wa', 'Walloon', 'walon'),
(176, 'wo', 'Wolof', 'Wollof'),
(177, 'xh', 'Xhosa', 'isiXhosa'),
(178, 'yi', 'Yiddish', 'ייִדיש'),
(179, 'yo', 'Yoruba', 'Yorùbá'),
(180, 'za', 'Zhuang', 'Saɯ cueŋƅ'),
(181, 'zh', 'Chinese', '中文'),
(182, 'zu', 'Zulu', 'isiZulu');
