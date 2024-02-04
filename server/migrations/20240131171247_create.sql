-- Add migration script here

CREATE TABLE IF NOT EXISTS items 
(
    id INTEGER PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name VARCHAR(32) NOT NULL,
    title VARCHAR(128) NOT NULL,
    created DATETIME NOT NULL default current_timestamp,
    updated DATETIME NOT NULL default current_timestamp on update current_timestamp,
    price_type VARCHAR(16) NOT NULL,
    price DOUBLE NOT NULL,
    location POINT NOT NULL,
    place_description VARCHAR(128) NOT NULL,
    category VARCHAR(16) NOT NULL,
    subcategory VARCHAR(16) NOT NULL,
    user VARCHAR(16) NOT NULL,
    reserved VARCHAR(16),
    status VARCHAR(16) NOT NULL
);

CREATE TABLE IF NOT EXISTS users
(
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    name VARCHAR(32) NOT NULL, title VARCHAR(128)        NOT NULL,
    email VARCHAR(128)        NOT NULL,
    about VARCHAR(128)        NOT NULL,
    avatar VARCHAR(128)        NOT NULL,
    joined DATETIME        NOT NULL default current_timestamp,
    last_login DATETIME        NOT NULL on update current_timestamp
);

CREATE TABLE IF NOT EXISTS reservations
(
    id          INTEGER PRIMARY KEY NOT NULL AUTO_INCREMENT,
    item INTEGER NOT NULL,
    user VARCHAR(16) NOT NULL,
    message VARCHAR(128)        NOT NULL,
    avatar VARCHAR(128)        NOT NULL,
    created DATETIME        NOT NULL default current_timestamp,
    last_login DATETIME        NOT NULL on update current_timestamp
);
