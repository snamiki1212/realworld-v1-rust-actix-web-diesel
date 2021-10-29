CREATE TABLE comments (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  article_id UUID NOT NULL REFERENCES articles (id) ON DELETE CASCADE,
  author_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  body TEXT NOT NULL,
  create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX comments_article_id_idx ON comments (article_id);
CREATE INDEX comments_author_id_idx ON comments (author_id);
