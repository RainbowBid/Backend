CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Creating an Enum type for Category
CREATE TYPE Category AS ENUM ('art', 'sport', 'electronics', 'services');

CREATE TABLE Users (
                       id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                       username text,
                       email text,
                       password text
);

CREATE TABLE Items (
                       id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                       brief text,
                       description text,
                       picture bytea,
                       user_id uuid,
                       FOREIGN KEY (user_id) REFERENCES Users(id)
);

CREATE TABLE ItemsCategory (
                               item_id uuid,
                               tag Category,
                               PRIMARY KEY (item_id, tag),
                               FOREIGN KEY (item_id) REFERENCES Items(id)
);

CREATE TABLE Auctions (
                          id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                          starting_price real,
                          end_date timestamp,
                          item_id uuid,
                          FOREIGN KEY (item_id) REFERENCES Items(id)
);

CREATE TABLE Bids (
                      id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                      value real,
                      auction_id uuid,
                      user_id uuid,
                      FOREIGN KEY (auction_id) REFERENCES Auctions(id),
                      FOREIGN KEY (user_id) REFERENCES Users(id)
);
