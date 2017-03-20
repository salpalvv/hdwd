create table products (
  id            integer not null primary key,
  name          text not null,
  artifact      text not null,
  environment   text not null,
  version       text not null,
  deployed      text not null
)
