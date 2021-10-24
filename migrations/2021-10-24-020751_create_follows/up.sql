CREATE TABLE follows (
  followee_id UUID NOT NULL REFERENCES users (id),
  follower_id UUID NOT NULL REFERENCES users (id),
  PRIMARY KEY (follower_id, followee_id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX follows_follower_id ON follows (follower_id);
CREATE INDEX follows_followee_id ON follows (followee_id);

ALTER TABLE follows
  ADD CONSTRAINT follower_id_cannot_be_equal_to_followee_id
  CHECK (follower_id != followee_id);
