DROP TABLE IF EXISTS methods;

CREATE TABLE methods (
    id INT AUTO_INCREMENT,
    name CHAR(3),
    description VARCHAR(4095),
    image VARCHAR(2083),
    CONSTRAINT PRIMARY KEY (id)
);

INSERT INTO methods
    (name, description, image)
VALUES
    ("200", "Ok", "thumbs up emoji"),
    ("204", "No Content", "empty nest"),
    ("301", "Moved Permanently", "abandoned house or boarded up storefront"),
    ("302", "Moved Temporarily", "out to lunch sign or one of those clocks with, like, a back-at-eight thingy"),
    ("403", "Forbidden", "Emoji spam for no"),
    ("404", "Not found", "Rotating thoughtful emoji head"),
    ("405", "Method Not Allowed", "That meme where the lady is putting bridge shape in square hole"),
    ("413", "Payload Too Large", "Someone pushing something too large into osmething too small"),
    ("418", "I'm a teapot", "Teapot"),
    ("500", "Internal Server Error", "Server with smoke coming out of it. Or car crash"),
    ("501", "Method Not Known", "Dog meme or something. AnimalsBeingDerps subreddit probably")
;
