create table credentials (
        username text not null primary key,
        password text not null,
        salt text not null,
        admin boolean not null default false
);
