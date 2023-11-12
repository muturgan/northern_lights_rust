CREATE TABLE "promo" (
	"promocode"     VARCHAR(12),
	"holder_id"     INT NOT NULL,
	"activated_at"  TIMESTAMPTZ(3)  DEFAULT NULL,
	"created_at"    TIMESTAMPTZ(3)  DEFAULT CURRENT_TIMESTAMP,

	CONSTRAINT "PK_promo" PRIMARY KEY ("promocode"),
	CONSTRAINT "FK_promo-to-users" FOREIGN KEY ("holder_id") REFERENCES "users"("id") ON DELETE NO ACTION
);
