CREATE TABLE if not exists public.image_embeddings
(
    id          serial4       NOT NULL,
    image_path  text          NOT NULL,
    description text          NOT NULL,
    embedding   public.vector NULL,
    CONSTRAINT image_embeddings_pkey PRIMARY KEY (id)
);