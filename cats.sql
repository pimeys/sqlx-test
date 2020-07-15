--
-- PostgreSQL database dump
--

-- Dumped from database version 10.11 (Debian 10.11-1.pgdg90+1)
-- Dumped by pg_dump version 12.2 (Ubuntu 12.2-4)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: Mood; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public."Mood" AS ENUM (
    'HAPPY',
    'HUNGRY'
);


ALTER TYPE public."Mood" OWNER TO postgres;

SET default_tablespace = '';

--
-- Name: Cat; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Cat" (
    id text NOT NULL,
    mood public."Mood" NOT NULL
);


ALTER TABLE public."Cat" OWNER TO postgres;

--
-- Name: Human; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Human" (
    id text NOT NULL,
    mood public."Mood" NOT NULL
);


ALTER TABLE public."Human" OWNER TO postgres;

--
-- PostgreSQL database dump complete
--

