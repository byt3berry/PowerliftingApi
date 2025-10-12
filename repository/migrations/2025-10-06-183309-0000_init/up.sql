CREATE TYPE Federation AS ENUM (
    'FFFORCE',
    'EPF',
    'IPF',
    'FFHMFAC',
    'OTHER'
);

CREATE TYPE Country AS ENUM (
    'FRANCE',
    'OTHER'
);

CREATE TYPE Division AS ENUM (
    'Any',
    'Open',
    'G',
    'Cadet',
    'Elite',
    'SubJuniors',
    'Juniors',
    'Seniors',
    'Masters',
    'Masters1',
    'Masters2',
    'Masters3',
    'Masters4'
);

CREATE TYPE Equipment AS ENUM (
    'Any',
    'Raw',
    'Wraps',
    'Single',
    'Multi',
    'Straps',
    'Sleeves',
    'Bare',
    'Unlimited'
);

CREATE TYPE Sex AS ENUM ('M', 'F');

CREATE TYPE Weight AS (
    weight NUMERIC(8, 4)
);

CREATE TYPE WeightClassKind AS ENUM (
    'UnderOrEqual',
    'Over',
    'None'
);

CREATE TYPE WeightClass AS (
    kind WeightClassKind,
    weight Weight
);

CREATE TABLE Meets (
    id SERIAL PRIMARY KEY UNIQUE,
    name VARCHAR(256) NOT NULL,
    federation Federation NOT NULL,
    country Country NOT NULL,
    state VARCHAR(256) NOT NULL,
    town VARCHAR(256) NOT NULL
);

CREATE TABLE Entries (
    id SERIAL PRIMARY KEY UNIQUE,
    meet_id INTEGER REFERENCES meets(id) NOT NULL,
    name VARCHAR(256) NOT NULL,
    division Division NOT NULL,
    equipment Equipment NOT NULL,
    sex Sex NOT NULL,
    bodyweight Weight NOT NULL,
    weight_class WeightClass NOT NULL,
    squat1 Weight NOT NULL,
    squat2 Weight NOT NULL,
    squat3 Weight NOT NULL,
    squat4 Weight NOT NULL,
    bench1 Weight NOT NULL,
    bench2 Weight NOT NULL,
    bench3 Weight NOT NULL,
    bench4 Weight NOT NULL,
    deadlift1 Weight NOT NULL,
    deadlift2 Weight NOT NULL,
    deadlift3 Weight NOT NULL,
    deadlift4 Weight NOT NULL,
    best_squat Weight NOT NULL,
    best_bench Weight NOT NULL,
    best_deadlift Weight NOT NULL,
    total Weight NOT NULL
);
