--
-- PostgreSQL database dump
--

-- Dumped from database version 17.5 (Debian 17.5-1.pgdg120+1)
-- Dumped by pg_dump version 17.5 (Debian 17.5-1.pgdg120+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: users; Type: TABLE; Schema: public; Owner: vax
--

CREATE TABLE public.users (
    id bigserial NOT NULL,
    age integer,
    name character varying(255) NOT NULL,
    firstname character varying(255) NOT NULL
);


ALTER TABLE public.users OWNER TO demo;

--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: vax
--

COPY public.users (id, age, name, firstname) FROM stdin;
1	25	BARAIS	OLIVIER
2	23	Test	Test1
\.


--
-- Name: users USERS_pkey; Type: CONSTRAINT; Schema: public; Owner: vax
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT "USERS_pkey" PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

