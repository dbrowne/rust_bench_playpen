-- Your SQL goes here
create table items
(
    id   bigint primary key not null,
    event text               not null,
    c_time timestamp          not null,
    m_time timestamp          not null
);
