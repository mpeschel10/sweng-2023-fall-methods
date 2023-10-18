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
    ("204", "No Content", "/images/204.png"),
    ("200", "Ok", "/images/200.jpg"),
    ("302", "Moved Temporarily", "/images/302.png"),
    ("301", "Moved Permanently", "/images/301.jpg"),
    ("418", "I'm a teapot", "/images/418.webp"),
    ("413", "Payload Too Large", "/images/413.jpg.webp"),
    ("405", "Method Not Allowed", "/images/405.png"),
    ("404", "Not found", "/images/404.jpg"), -- Actually "/images/404.gif"
    ("403", "Forbidden", "/images/403.png"),
    ("501", "Method Not Known", "/images/501.jpg"),
    ("500", "Internal Server Error", "/images/500.gif")
;
