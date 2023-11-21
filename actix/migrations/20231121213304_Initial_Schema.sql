CREATE SCHEMA IF NOT EXISTS rcv;

CREATE TABLE IF NOT EXISTS rcv.User (
  UserId INT NOT NULL GENERATED ALWAYS AS IDENTITY CONSTRAINT PK_User PRIMARY KEY,
  UserName VARCHAR(64) NOT NULL CONSTRAINT UX_UserName UNIQUE,
  DisplayName VARCHAR(256)
);

CREATE TABLE IF NOT EXISTS rcv.Poll (
  PollId INT NOT NULL GENERATED ALWAYS AS IDENTITY CONSTRAINT PK_Poll PRIMARY KEY,
  Title VARCHAR(256) NOT NULL,
  Description VARCHAR(1024),
  CreatedByUserId INT REFERENCES rcv.User(UserId),
  CreatedTimestamp TIMESTAMP WITH TIME ZONE NOT NULL,
  CloseTimestamp TIMESTAMP WITH TIME ZONE NOT NULL,
  Rounds INT
);

CREATE TABLE IF NOT EXISTS rcv.PollOption (
  PollOptionId INT NOT NULL GENERATED ALWAYS AS IDENTITY CONSTRAINT PK_PollOption PRIMARY KEY,
  PollId INT NOT NULL REFERENCES rcv.Poll(PollId),
  Title Varchar(256) NOT NULL,
  Description VARCHAR(1024),
  CONSTRAINT UX_Poll_Option_Title UNIQUE (PollId, Title)
);
CREATE INDEX IF NOT EXISTS IX_PollOption_Poll ON rcv.PollOption (PollId);

ALTER TABLE rcv.Poll ADD COLUMN IF NOT EXISTS WinnerId INT;
ALTER TABLE rcv.Poll ADD CONSTRAINT FK_Poll_PollOptionWinner FOREIGN KEY (WinnerId) REFERENCES rcv.PollOption(PollOptionId);

CREATE TABLE IF NOT EXISTS rcv.Vote (
  VoteId INT NOT NULL GENERATED ALWAYS AS IDENTITY CONSTRAINT PK_Vote PRIMARY KEY,
  UserId INT NOT NULL REFERENCES rcv.User(UserId),
  PollId INT NOT NULL REFERENCES rcv.Poll(PollId),
  PollOptionId INT NOT NULL REFERENCES rcv.PollOption(PollOptionId),
  Ordinal INT NOT NULL,
  CONSTRAINT UX_Vote_Poll_Ordinal UNIQUE (UserId, PollId, Ordinal),
  CONSTRAINT UX_Vote_User_PollOption UNIQUE (UserId, PollOptionId, PollId)
);
CREATE INDEX IF NOT EXISTS IX_Vote_User_Poll ON rcv.Vote (UserId, PollId);

CREATE OR REPLACE FUNCTION rcv.AddUser
( userName rcv.User.UserName%TYPE
, displayName rcv.User.DisplayName%TYPE
) RETURNS rcv.User.UserId%TYPE
AS $$
   INSERT INTO rcv.User (UserName, DisplayName)
   VALUES (userName, displayName)
   RETURNING UserId
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION rcv.StartPoll
( userId rcv.User.UserId%TYPE
, title rcv.Poll.Title%TYPE
, description rcv.Poll.Description%TYPE
, createdTimestamp rcv.Poll.CreatedTimestamp%TYPE
, closeTimestamp rcv.Poll.CloseTimestamp%TYPE
) RETURNS rcv.Poll.PollId%TYPE
AS $$
    INSERT INTO rcv.Poll (Title, Description, CreatedTimestamp, CloseTimestamp, CreatedByUserId)
    VALUES (title, description, createdTimestamp, closeTimestamp, userId)
    RETURNING PollId
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION rcv.AddPollOption
( pollId rcv.Poll.PollId%TYPE
, title rcv.PollOption.Title%TYPE
, description rcv.PollOption.Description%TYPE
) RETURNS rcv.PollOption.PollOptionId%TYPE
AS $$
BEGIN
   IF EXISTS (SELECT FROM rcv.Poll WHERE PollId = pollId AND CloseTimestamp < NOW()) THEN
      RAISE EXCEPTION 'Cannot alter poll options after poll closes';
   ELSE
      INSERT INTO rcv.PollOption (PollId, Title, Description)
      VALUES (pollId, title, description)
      RETURNING PollOptionId;
   END IF;
END
$$ LANGUAGE PLPGSQL;

CREATE OR REPLACE FUNCTION rcv.RemovePollOption
( pollOptionId rcv.PollOption.PollOptionId%TYPE
) RETURNS BIGINT
AS $$
DECLARE
  total_affected BIGINT = 0;
BEGIN
   IF NOT EXISTS (SELECT FROM rcv.PollOption WHERE PollOptionId = pollOptionId) THEN
      RAISE EXCEPTION 'Option does not exist';
   ELSEIF EXISTS (SELECT FROM rcv.Poll WHERE PollId = pollId AND CloseTimestamp < NOW()) THEN
      RAISE EXCEPTION 'Cannot alter poll options after poll closes';
   ELSE
      WITH votes AS (DELETE FROM rcv.Vote WHERE PollOptionId = pollOptionId RETURNING *)
      SELECT total_affected = COUNT(*) FROM votes;
      WITH poll_option AS (DELETE FROM rcv.PollOption WHERE PollOptionId = pollOptionId RETURNING *)
      SELECT total_affected = total_affected + COUNT(*) FROM poll_option;
      RETURN total_affected;
   END IF;
END
$$ LANGUAGE PLPGSQL;
