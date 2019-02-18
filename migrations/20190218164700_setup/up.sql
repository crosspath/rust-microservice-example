create table users (
  id serial primary key not null,
  email varchar(255) not null,
  created_at timestamp not null,
  updated_at timestamp not null
);

create table user_orders (
  id serial primary key not null,
  user_id integer not null references users(id),
  product varchar(255) not null,
  price float not null,
  created_at timestamp not null,
  updated_at timestamp not null
);

create table bonus_accounts (
  id serial primary key not null,
  user_id integer not null references users(id),
  bonuses float not null default 0,
  created_at timestamp not null,
  updated_at timestamp not null
);

create table bonus_logs (
  id serial primary key not null,
  bonus_account_id integer not null references bonus_accounts(id),
  user_order_id integer not null references user_orders(id),
  bonuses float not null default 0,
  created_at timestamp not null,
  updated_at timestamp not null
);

comment on column bonus_logs.bonus_account_id is
'Who will receive bonus for inviting new user with this UserOrder?';
