import { Orchestrator, Config } from "@holochain/tryorama";

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

const orchestrator = new Orchestrator();

export const simpleConfig = {
  alice: Config.dna("../mutual_commitments.dna.gz", null),
  bobbo: Config.dna("../mutual_commitments.dna.gz", null),
  carol: Config.dna("../mutual_commitments.dna.gz", null),
};

orchestrator.registerScenario(
  "create and get a calendar event",
  async (s, t) => {
    const { conductor } = await s.players({
      conductor: Config.gen(simpleConfig),
    });
    await conductor.spawn();

    await conductor.call("alice", "profiles", "create_profile", {
      username: "alice",
      fields: {},
    });
    let profileId = await conductor.call(
      "alice",
      "profiles",
      "get_my_profile_entry_hash",
      null
    );

    let aliceAddress = await conductor.call(
      "alice",
      "mutual_commitments",
      "who_am_i",
      null
    );

    let bobAddress = await conductor.call(
      "bobbo",
      "mutual_commitments",
      "who_am_i",
      null
    );

    let carolAddress = await conductor.call(
      "carol",
      "mutual_commitments",
      "who_am_i",
      null
    );

    await conductor.call(
      "bobbo",
      "mutual_commitments",
      "invite_agents_to_commit",
      {
        entry_hash: profileId,
        agents_to_invite: [aliceAddress, carolAddress],
      }
    );
    await sleep(10);

    let invitations = await conductor.call(
      "alice",
      "mutual_commitments",
      "get_my_invitations",
      null
    );
    t.equal(invitations.length, 1);
    t.equal(invitations[0], profileId);

    await conductor.call(
      "alice",
      "mutual_commitments",
      "accept_invitation_and_commit",
      profileId
    );

    await sleep(10);

    invitations = await conductor.call(
      "alice",
      "mutual_commitments",
      "get_my_invitations",
      null
    );
    t.equal(invitations.length, 0);

    let commitments = await conductor.call(
      "alice",
      "mutual_commitments",
      "get_my_commitments",
      null
    );
    t.equal(commitments.length, 1);
    t.equal(commitments[0], profileId);
  }
);

orchestrator.run();
