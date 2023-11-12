CREATE TABLE "users" (
	"id"         INT  GENERATED ALWAYS AS IDENTITY,
	"firstname"  VARCHAR(32) NOT NULL,
	"birthdate"  DATE NOT NULL,
	"phone"      CHAR(12) NOT NULL,
	"email"      VARCHAR(32) DEFAULT NULL,
	"created_at" TIMESTAMPTZ(3) DEFAULT CURRENT_TIMESTAMP,

	CONSTRAINT "UQ_users:phone" UNIQUE ("phone"),
	CONSTRAINT "UQ_users:email" UNIQUE ("email"),
	CONSTRAINT "PK_users" PRIMARY KEY ("id")
);
