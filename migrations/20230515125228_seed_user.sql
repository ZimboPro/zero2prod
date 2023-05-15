-- Add migration script here

INSERT INTO users (user_id, username, password_hash)
VALUES (
'ddf8994f-d522-4659-8d02-c1d479057be6',
'admin',
'$argon2id$v=19$m=15000,t=2,p=1$V8HbtnmYp6pRC/x1+kw11A$9Cx12UMbeG99hFrAaqZjpVw1XvQvU0hHn7wU1YSGBko'
);
