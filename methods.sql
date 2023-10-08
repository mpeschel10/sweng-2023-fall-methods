DROP TABLE IF EXISTS methods;

CREATE TABLE methods (
    id INT AUTO_INCREMENT,
    name INT,
    description VARCHAR(4095),
    image VARCHAR(2083),
    CONSTRAINT PRIMARY KEY (id)
);

-- INSERT INTO methods
--     (name, description, image)
-- VALUES
--     ("200", ),
--     ();
