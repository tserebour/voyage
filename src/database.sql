

CREATE TABLE voyage_users (
  id SERIAL PRIMARY KEY,
  fullname VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  phone_number VARCHAR(20) NOT NULL UNIQUE,
  account_created_at VARCHAR(255) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_login_at VARCHAR(255)
);

CREATE TABLE ride_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    base_fare NUMERIC(10, 2) NOT NULL,
    rate_per_mile NUMERIC(10, 2) NOT NULL,
    rate_per_minute NUMERIC(10, 2) NOT NULL
);


CREATE TABLE voyage_drivers (
  id SERIAL PRIMARY KEY,
  fullname VARCHAR(255) NOT NULL, 
  email VARCHAR(255) NOT NULL UNIQUE, 
  password VARCHAR(255) NOT NULL,
  license_number TEXT,
  vehicle_information TEXT,
  rating VARCHAR(255) DEFAULT '0',
  earn_type INT,
  current_latitude DOUBLE PRECISION NULL,
  current_longitude DOUBLE PRECISION NULL



);

ALTER TABLE voyage_drivers ADD COLUMN earn_type_id integer;


ALTER TABLE voyage_drivers
ADD COLUMN current_latitude DOUBLE PRECISION NULL,
ADD COLUMN current_longitude DOUBLE PRECISION NULL;





CREATE TABLE cars (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    capacity INT NOT NULL,
    ride_type_id INT NOT NULL REFERENCES ride_types(id),
    driver_id INT NOT NULL REFERENCES voyage_drivers(id)
);

CREATE TABLE driver_locations (
    id SERIAL PRIMARY KEY,
    driver_id INT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    previous_latitude DOUBLE PRECISION,
    previous_longitude DOUBLE PRECISION,
    timestamp VARCHAR(255) DEFAULT CURRENT_TIMESTAMP
);



CREATE TABLE ride_requests (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES voyage_users(id),
    pickup_address TEXT NOT NULL,
    pickup_latitude FLOAT NOT NULL,
    pickup_longitude FLOAT NOT NULL,
    dropoff_address TEXT NOT NULL,
    dropoff_latitude FLOAT NOT NULL,
    dropoff_longitude FLOAT NOT NULL,
    ride_type_id INT NOT NULL REFERENCES ride_types(id),
    estimated_fare NUMERIC(10, 2),
    requested_at VARCHAR(255) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'PENDING'
);

CREATE TABLE rides (
    id SERIAL PRIMARY KEY,
    ride_request_id INT NOT NULL REFERENCES ride_requests(id),
    driver_id INT NOT NULL REFERENCES voyage_users(id),
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    actual_fare NUMERIC(10, 2),
    rating INT,
    feedback TEXT
);











CREATE TABLE bra_fie_users(
  id SERIAL PRIMARY KEY,
  fullname VARCHAR(255) NOT NULL, 
  email VARCHAR(255) NOT NULL UNIQUE, 
  phone VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL  
);

INSERT INTO drivers_users (fullname, email, phone, password) VALUES ("Emmanuel Tabi", tserebour1@gmail.com,$3, $4) RETURNING id,fullname, email,phone, password


CREATE OR REPLACE FUNCTION notify_driver_location()
RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('driver_location', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER driver_location_notify
AFTER INSERT ON driver_locations
FOR EACH ROW
EXECUTE FUNCTION notify_driver_location();
