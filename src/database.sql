-- CREATE TABLE voyage_users (
--   id SERIAL PRIMARY KEY,
--   fullname VARCHAR(255) NOT NULL, 
--   email VARCHAR(255) NOT NULL UNIQUE, 
--   password VARCHAR(255) NOT NULL
    
-- );


CREATE TABLE voyage_users (
  id SERIAL PRIMARY KEY,
  fullname VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  phone_number VARCHAR(20) NOT NULL UNIQUE,
  account_created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_login_at TIMESTAMP
);

CREATE TABLE drivers_users (
  id SERIAL PRIMARY KEY,
  fullname VARCHAR(255) NOT NULL, 
  email VARCHAR(255) NOT NULL UNIQUE, 
  password VARCHAR(255) NOT NULL

);


CREATE TABLE bra_fie_users(
  id SERIAL PRIMARY KEY,
  fullname VARCHAR(255) NOT NULL, 
  email VARCHAR(255) NOT NULL UNIQUE, 
  phone VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL  
);

INSERT INTO drivers_users (fullname, email, phone, password) VALUES ("Emmanuel Tabi", tserebour1@gmail.com,$3, $4) RETURNING id,fullname, email,phone, password
