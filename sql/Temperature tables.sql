CREATE TABLE IF NOT EXISTS room(
    id INT GENERATED ALWAYS AS IDENTITY,
    room_name varchar(50) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS temperature(
  id integer GENERATED ALWAYS AS IDENTITY,
  room_id integer NOT NULL,
  temperature decimal NOT NULL,
  humidity decimal NOT NULL,
  time TIMESTAMP NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT fk_room
    FOREIGN KEY(room_id)
        REFERENCES room(id)
);

INSERT INTO room (room_name)
VALUES
    ('Family Room'),
    ('Living Room'),
    ('Kitchen'),
    ('Master bedroom'),
    ('Cora and Izzy bedroom'),
    ('Sophia bedroom'),
    ('Office'),
    ('Garage');
