-- https://docs.rs/sqlx-pg-migrate/latest/sqlx_pg_migrate/
-- watching the migrations dir https://docs.rs/sqlx/latest/sqlx/macro.migrate.html

-- Create tables for the Bookstore API

-- Authors table
CREATE TABLE IF NOT EXISTS authors (
    id TEXT PRIMARY KEY,
    title TEXT,
    first_name TEXT NOT NULL,
    second_names TEXT[],
    last_name TEXT NOT NULL,
    date_of_birth DATE NOT NULL,
    date_of_death DATE CHECK (date_of_death IS NULL OR date_of_death >= date_of_birth)
);

-- Genres table
CREATE TABLE IF NOT EXISTS genres (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Discount codes table
CREATE TABLE IF NOT EXISTS discount_codes (
    id TEXT PRIMARY KEY,
    percentage_discount INTEGER NOT NULL CHECK (percentage_discount BETWEEN 0 AND 100),
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL CHECK (valid_to >= valid_from),
    code TEXT NOT NULL UNIQUE
);

-- Books table
CREATE TABLE IF NOT EXISTS books (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    release DATE NOT NULL,
    first_release DATE NOT NULL CHECK (first_release <= release),
    series TEXT,
    edition INTEGER NOT NULL CHECK (edition > 0),
    price DECIMAL(10, 2) NOT NULL CHECK (price >= 0),
    available INTEGER NOT NULL CHECK (available >= 0),
    status TEXT NOT NULL CHECK (status IN ('available', 're-ordered', 'out-of-stock'))
);

-- Book authors relationship table
CREATE TABLE IF NOT EXISTS book_authors (
    book_id TEXT REFERENCES books(id) ON DELETE CASCADE,
    author_id TEXT REFERENCES authors(id) ON DELETE RESTRICT,
    PRIMARY KEY (book_id, author_id)
);

-- Book genres relationship table
CREATE TABLE IF NOT EXISTS book_genres (
    book_id TEXT REFERENCES books(id) ON DELETE CASCADE,
    genre_id TEXT REFERENCES genres(id) ON DELETE RESTRICT,
    PRIMARY KEY (book_id, genre_id)
);

-- Book discount codes relationship table
CREATE TABLE IF NOT EXISTS book_discounts (
    book_id TEXT REFERENCES books(id) ON DELETE CASCADE,
    discount_id TEXT REFERENCES discount_codes(id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, discount_id)
);

-- Addresses table
CREATE TABLE IF NOT EXISTS addresses (
    id SERIAL PRIMARY KEY,
    street TEXT NOT NULL,
    street_number TEXT NOT NULL,
    zip_code TEXT NOT NULL,
    city TEXT NOT NULL,
    province TEXT,
    country TEXT NOT NULL
);

-- Orders table
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    shipping_date DATE NOT NULL,
    billing_address_id INTEGER REFERENCES addresses(id) ON DELETE RESTRICT NOT NULL,
    shipping_address_id INTEGER REFERENCES addresses(id) ON DELETE SET NULL,
    status TEXT NOT NULL CHECK (status IN ('placed', 'shipped', 'delivered', 'canceled'))
);

-- Order items table
CREATE TABLE IF NOT EXISTS order_items (
    order_id TEXT REFERENCES orders(id) ON DELETE CASCADE,
    book_id TEXT REFERENCES books(id) ON DELETE RESTRICT,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (order_id, book_id)
);

-- Create triggers for data integrity

-- Add function to check if book can be deleted
CREATE OR REPLACE FUNCTION check_book_deletion() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM order_items WHERE book_id = OLD.id) THEN
        RAISE EXCEPTION 'Cannot delete book that is part of order history';
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_book_deletion_trigger
BEFORE DELETE ON books
FOR EACH ROW
EXECUTE FUNCTION check_book_deletion();

-- Add function to check if author can be deleted
CREATE OR REPLACE FUNCTION check_author_deletion() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM book_authors WHERE author_id = OLD.id) THEN
        RAISE EXCEPTION 'Cannot delete author that has books in the catalog';
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_author_deletion_trigger
BEFORE DELETE ON authors
FOR EACH ROW
EXECUTE FUNCTION check_author_deletion();

-- Add function to check if genre can be deleted
CREATE OR REPLACE FUNCTION check_genre_deletion() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM book_genres WHERE genre_id = OLD.id) THEN
        RAISE EXCEPTION 'Cannot delete genre that is assigned to books in the catalog';
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_genre_deletion_trigger
BEFORE DELETE ON genres
FOR EACH ROW
EXECUTE FUNCTION check_genre_deletion();

-- Add trigger to prevent address deletion if used by orders
CREATE OR REPLACE FUNCTION prevent_address_delete() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM orders WHERE billing_address_id = OLD.id) THEN
        RAISE EXCEPTION 'Cannot delete address used as billing address in orders';
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_address_delete_trigger
BEFORE DELETE ON addresses
FOR EACH ROW
EXECUTE FUNCTION prevent_address_delete();

-- Trigger to update book status based on available quantity
CREATE OR REPLACE FUNCTION update_book_status() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.available <= 0 THEN
        NEW.status = 'out-of-stock';
    ELSIF NEW.available <= 5 THEN
        NEW.status = 're-ordered';
    ELSE
        NEW.status = 'available';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_book_status_trigger
BEFORE INSERT OR UPDATE OF available ON books
FOR EACH ROW
EXECUTE FUNCTION update_book_status();

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_books_status ON books(status);
CREATE INDEX IF NOT EXISTS idx_books_title ON books(title);
CREATE INDEX IF NOT EXISTS idx_books_series ON books(series) WHERE series IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_books_price ON books(price);
CREATE INDEX IF NOT EXISTS idx_books_release ON books(release);

CREATE INDEX IF NOT EXISTS idx_authors_names ON authors(first_name, last_name);
CREATE INDEX IF NOT EXISTS idx_authors_birth ON authors(date_of_birth);

CREATE INDEX IF NOT EXISTS idx_discount_codes_valid_dates ON discount_codes(valid_from, valid_to);
CREATE INDEX IF NOT EXISTS idx_discount_codes_code ON discount_codes(code);

CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_customer_id ON orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_orders_shipping_date ON orders(shipping_date);

-- Indexes for foreign keys
CREATE INDEX IF NOT EXISTS idx_book_authors_book_id ON book_authors(book_id);
CREATE INDEX IF NOT EXISTS idx_book_authors_author_id ON book_authors(author_id);

CREATE INDEX IF NOT EXISTS idx_book_genres_book_id ON book_genres(book_id);
CREATE INDEX IF NOT EXISTS idx_book_genres_genre_id ON book_genres(genre_id);

CREATE INDEX IF NOT EXISTS idx_book_discounts_book_id ON book_discounts(book_id);
CREATE INDEX IF NOT EXISTS idx_book_discounts_discount_id ON book_discounts(discount_id);

CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_order_items_book_id ON order_items(book_id);