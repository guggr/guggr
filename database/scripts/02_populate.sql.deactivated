BEGIN;

--
-- Data for Name: group; Type: TABLE DATA; Schema: public; Owner: guggr
--
INSERT INTO
	public."group"
VALUES
	('CP4PUbAhorcKEHjdVZP_B', 'Test Group');

--
-- Data for Name: job_type; Type: TABLE DATA; Schema: public; Owner: guggr
--
INSERT INTO
	public.job_type
VALUES
	('http', 'HTTP');

INSERT INTO
	public.job_type
VALUES
	('ping', 'Ping');

--
-- Data for Name: job; Type: TABLE DATA; Schema: public; Owner: guggr
--
INSERT INTO
	public.job
VALUES
	(
		'L5tboqyp3NGfq-ZYlWEFg',
		'Check Google HTTP',
		'http',
		'CP4PUbAhorcKEHjdVZP_B',
		true,
		NULL,
		'00:01:00',
		NULL
	);

INSERT INTO
	public.job
VALUES
	(
		'EV0AwkfG4YKUTw2ULe1OW',
		'Check Google Ping',
		'ping',
		'CP4PUbAhorcKEHjdVZP_B',
		true,
		NULL,
		'00:01:00',
		NULL
	);

--
-- Data for Name: role; Type: TABLE DATA; Schema: public; Owner: guggr
--
INSERT INTO
	public.role
VALUES
	('user', 'User');

INSERT INTO
	public.role
VALUES
	('admin', 'Administrator');

INSERT INTO
	public.role
VALUES
	('owner', 'Owner');

--
-- Data for Name: user; Type: TABLE DATA; Schema: public; Owner: guggr
--
INSERT INTO
	public."user"
VALUES
	(
		'CzMoS6ybm95AVKw9LcEeQ',
		'Test User1',
		'user1@test.com',
		'a'
	);

INSERT INTO
	public."user"
VALUES
	(
		'umWxSSA7oOiN8zKZxBod2',
		'Test User2',
		'user2@test.com',
		'a'
	);

INSERT INTO
	public."user"
VALUES
	(
		'JxSTT9dHzUFCilMYetbSf',
		'Test User3',
		'user3@test.com',
		'a'
	);

--
-- Data for Name: user_group_mapping; Type: TABLE DATA; Schema: public; Owner: guggr
--
INSERT INTO
	public.user_group_mapping
VALUES
	(
		'CzMoS6ybm95AVKw9LcEeQ',
		'CP4PUbAhorcKEHjdVZP_B',
		'owner'
	);

INSERT INTO
	public.user_group_mapping
VALUES
	(
		'umWxSSA7oOiN8zKZxBod2',
		'CP4PUbAhorcKEHjdVZP_B',
		'admin'
	);

INSERT INTO
	public.user_group_mapping
VALUES
	(
		'JxSTT9dHzUFCilMYetbSf',
		'CP4PUbAhorcKEHjdVZP_B',
		'user'
	);

COMMIT;
