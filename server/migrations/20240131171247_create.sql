-- Add migration script here

CREATE TABLE IF NOT EXISTS items 
(
    id BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    title VARCHAR(256) NOT NULL,
    description VARCHAR(1024) NOT NULL,
    created DATETIME NOT NULL DEFAULT current_timestamp,
    updated DATETIME NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp,
    price_type VARCHAR(16) NOT NULL,
    price DOUBLE NOT NULL,
    location POINT NOT NULL,
    place_description VARCHAR(256),
    category VARCHAR(32) NOT NULL,
    subcategory VARCHAR(32) NOT NULL,
    image VARCHAR(128),
    user VARCHAR(32) NOT NULL,
    reserved VARCHAR(32),
    status VARCHAR(16),

    INDEX(category,subcategory),
    SPATIAL INDEX(location),
    FULLTEXT(title, description)
);

CREATE TABLE IF NOT EXISTS users
(
    id VARCHAR(32) PRIMARY KEY NOT NULL,
    name VARCHAR(32) NOT NULL,
    email VARCHAR(128) NOT NULL,
    about VARCHAR(1024),
    avatar VARCHAR(128),
    joined DATETIME NOT NULL DEFAULT current_timestamp,
    last_login DATETIME NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);

CREATE TABLE IF NOT EXISTS reservations
(
    id INTEGER PRIMARY KEY NOT NULL AUTO_INCREMENT,
    item INTEGER NOT NULL,
    user VARCHAR(32) NOT NULL,
    message VARCHAR(128) NOT NULL,
    created DATETIME NOT NULL DEFAULT current_timestamp
);

CREATE EVENT removeExpiredReservations
    ON SCHEDULE EVERY 5 MINUTE
DO
   DELETE FROM reservations
    WHERE TIMESTAMPDIFF(MINUTE, reservations.created, NOW()) > 5;

SET GLOBAL event_scheduler = ON;
