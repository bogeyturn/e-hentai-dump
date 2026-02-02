<script setup lang="ts">
import type { BountyInfo } from "exx";

defineProps<{ data: BountyInfo }>();
</script>

<template>
  <div id="b" class="stuffbox">
    <h1>{{ data.title }}</h1>

    <div id="t">
      <div id="b1">
        <div id="b2">
          <h2>WANTED</h2>
          <div>
            <img
              :src="data.img ?? '/g/wanted-blank.png'"
              style="margin: auto; border: 2px solid black"
              alt="Reward Image"
            />
          </div>
          <p>REWARD</p>
          <p>{{ data.reward }}</p>
        </div>
      </div>

      <div id="d">
        <table>
          <tbody>
            <tr>
              <td class="l">Bounty Posted By:</td>
              <td class="r">
                <NuxtLink :to="`/bounty?u=${data.posted_by_id}`"
                  >mikol</NuxtLink
                >
                <NuxtLink
                  :to="`https://forums.e-hentai.org/index.php?showuser=${data.posted_by_id}`"
                  ><img
                    class="ygm"
                    src="/g/ygm.png"
                    alt="PM"
                    title="Contact Poster"
                /></NuxtLink>
              </td>
            </tr>
            <tr>
              <td class="l">Posted Date:</td>
              <td class="r">
                {{ displayDate(data.posted) }}
                <template v-if="data.updated"
                  >(<strong>Updated</strong>:
                  {{ displayDate(data.updated) }})</template
                >
              </td>
            </tr>
            <tr>
              <td class="l">Bounty Type:</td>
              <td class="r">
                <strong>{{ data.type }}</strong>
              </td>
            </tr>
            <tr>
              <td class="l">Bounty Status:</td>
              <td class="r">
                <span
                  :class="{
                    scr: data.status == 'Closed/Completed',
                    sco:
                      data.status == 'Closed/Claimed' ||
                      data.status == 'Closed/Reserved',
                    scb: data.status == 'Open/Accepted',
                    scg: data.status == 'Open/New',
                  }"
                  >{{ data.status }}</span
                >
              </td>
            </tr>
            <tr>
              <td class="l">Min Hunter Rank:</td>
              <td class="r">
                <span :class="data.can_accept ? 'scg' : `scr`">{{
                  data.required_rank
                }}</span>
              </td>
            </tr>
            <tr>
              <td class="l">Current Reward:</td>
              <td class="r">{{ data.reward }}</td>
            </tr>
            <tr>
              <td class="l">Accepted Delivery:</td>
              <td class="r" v-html="data.accepted_delivery" />
            </tr>
          </tbody>
        </table>
        <div id="x" v-html="data.description" />
      </div>
    </div>

    <div id="c">
      <div id="a">
        <p>
          Bounties that are <span class="scg">Open/New</span> and
          <span class="scb">Open/Accepted</span> are open to be accepted and
          claimed. If you intent to fulfill a bounty within a reasonable amount
          of time, you can optionally <span class="scb">accept</span> the bounty
          first. These expire after a month.
        </p>
        <p>
          After a bounty has been <span class="sco">claimed</span>, the original
          poster of the bounty has seven days to accept or dispute it. If there
          is a dispute or the acceptance period expires, a Bounty Moderator will
          decide the outcome of the bounty.
        </p>
        <p>
          A rejected claim cannot be resubmitted, and will affect your rank.
        </p>
        <p>
          <strong
            >Your rank (Unranked) is insufficient to accept this bounty.</strong
          >
        </p>
        <form action="" method="post">
          <textarea
            name="ctext"
            cols="63"
            rows="10"
            placeholder="For accepting a bounty, you can enter a short comment here. For claiming a bounty, you must enter all the necessary details for where the bounty can be found."
          />
          <p style="font-weight: bold">
            If you intend to claim this bounty, make sure that all necessary
            URLs entered above are correct, and that they match the accepted
            delivery methods of this bounty. Do not, for instance, submit a link
            to a torrent file if that delivery method is not accepted.
          </p>
          <p style="font-weight: bold">
            All information required to validate your claim MUST be posted in
            the claim itself. The submitted claim MUST match all additional
            requirements stated in the bounty descrption
          </p>
          <p style="font-weight: bold">
            The submitted claim MUST have reasonable resolution and quality, and
            otherwise meet any quality requirements in the description.
          </p>
          <p style="font-weight: bold">
            For translation bounties, if you are submitting a machine
            translation, this MUST be explicitly allowed in the bounty
            description or by direct agreement with the bounty poster.
          </p>
          <p style="font-weight: bold">
            For editing bounties, the edits MUST be of reasonable quality, and
            you MUST use reasonable typesetting with an easily readable font.
          </p>
          <p style="font-weight: bold">
            In order to claim a bounty, you have to post a deposit of 1000
            credits. This is returned to you if the claim is accepted, but if
            the claim is found to be invalid, it will be forfeit.
          </p>
          <div id="n">
            <input
              type="submit"
              name="claimbounty"
              value="Accept Bounty"
              :disabled="!data.can_accept"
            /><input
              type="submit"
              name="claimbounty"
              value="Claim Bounty"
              :disabled="!data.can_accept"
            />
          </div>
        </form>
      </div>

      <div id="g">
        <p>
          You can add additional rewards as long as the bounty has not been
          accepted or claimed. Rewards lock in after 15 minutes and can then
          only be rescinded after a month or if the bounty is cancelled. You
          will have no saying in whether a claim is accepted or not.
        </p>

        <p>
          You have
          <strong
            >{{ data.credits_owned.toLocaleString("en-US") }} Credits</strong
          >
          and
          <strong>{{ data.hath_owned.toLocaleString("en-US") }} Hath</strong>.
          Minimum is <strong>5000 Credits</strong> or <strong>1 Hath</strong>.
        </p>
        <form action="" method="post">
          <p>
            <input
              type="text"
              name="grant_cred"
              value="0"
              size="10"
              maxlength="10"
            />
            C +
            <input
              type="text"
              name="grant_hath"
              value="0"
              size="10"
              maxlength="10"
            />
            Hath
            <input
              type="submit"
              name="addreward"
              value="Submit Additional Reward"
              style="width: 140px"
            />
          </p>
        </form>

        <table>
          <tbody>
            <tr>
              <th style="width: 95px">Grant Date</th>
              <th style="width: 150px">Amount</th>
              <th>Added By</th>
              <th style="width: 50px" />
            </tr>
            <tr v-for="(item, i) in data.reward_detail" :key="i">
              <td>
                {{ item.date ? displayDate(item.date) : "Original Bounty" }}
              </td>
              <td>{{ item.amount }}</td>
              <td>
                <NuxtLink
                  target="_forums"
                  :to="`https://forums.e-hentai.org/index.php?showuser=${item.added_by_id}`"
                >
                  {{ item.added_by }}
                </NuxtLink>
              </td>
              <td />
            </tr>
          </tbody>
        </table>
      </div>

      <div id="h">
        <table>
          <tbody>
            <tr>
              <th style="width: 95px">Claim Date</th>
              <th style="width: 110px">Status</th>
              <th>Bounty Hunter</th>
              <th style="width: 80px">Rating</th>
            </tr>
            <tr v-if="data.accepted.length == 0">
              <td colspan="4" class="bcn">
                This bounty has not been accepted or claimed by anyone.
              </td>
            </tr>
            <template v-if="data.accepted.length > 0">
              <template v-for="(item, i) in data.accepted" :key="i">
                <tr>
                  <td class="c1">{{ displayDate(item.date) }}</td>
                  <td class="c2">
                    <span
                      :class="{
                        scb: 'Bounty Accepted' == item.status,
                        scg: 'Claim Accepted' == item.status,
                        scr:
                          'Claim Accepted' != item.status &&
                          'Bounty Accepted' != item.status,
                      }"
                      >{{ item.status }}</span
                    >
                  </td>
                  <td class="c2">
                    <NuxtLink
                      target="_forums"
                      :to="`https://forums.e-hentai.org/index.php?showuser=${item.person_id}`"
                    >
                      {{ item.person }}
                    </NuxtLink>
                  </td>
                  <td class="c3">{{ item.rating }}</td>
                </tr>
                <tr>
                  <td
                    v-if="item.comment"
                    colspan="4"
                    class="c4"
                    v-html="item.comment"
                  />
                </tr>
              </template>
            </template>
          </tbody>
        </table>
      </div>

      <div class="c" />
    </div>
  </div>
</template>

<style scoped>
span.scr {
  font-weight: bold;
  color: red;
}

span.scb {
  font-weight: bold;
  color: blue;
}

span.scg {
  font-weight: bold;
  color: green;
}

span.sco {
  font-weight: bold;
  color: #ff8c00;
}

form {
  padding: 0;
  margin: 0;
}

div#b {
  width: 980px;
  margin: 5px auto;
  padding: 3px 3px 3px 0;
  font-size: 8pt;
}

div#b h1 {
  font-weight: bold;
  font-size: 12pt;
}

div#b1 {
  float: left;
  width: 460px;
  padding: 3px 3px 3px 0;
}

div#b2 {
  width: 456px;
  height: 585px;
  background-image: url(/g/wanted.png);
  background-repeat: no-repeat;
  padding: 25px 0 0 3px;
  font-family: impact, serif;
  font-size: 20pt;
}

div#b2 h2 {
  margin: 10px auto;
  padding: 0;
  font-size: 30pt;
}

div#b2 div {
  width: 400px;
  height: 400px;
  margin: auto;
}

div#b2 p {
  margin: 10px auto;
  padding: 0;
}

div#t {
  margin: 5px auto;
  padding: 3px;
}

div#d {
  float: left;
  width: 495px;
  padding: 3px 3px 3px 10px;
}

div#d a {
  text-decoration: none;
  font-weight: bold;
}

div#d table {
  text-align: left;
  width: 99%;
}

div#x {
  border: 1px solid #e3e0d1;
  padding: 5px 7px;
  height: 400px;
  overflow: auto;
  text-align: justify;
  border-radius: 9px;
  background: #f2efdf;
}

div#c {
  margin: 1px auto;
  padding: 1px 3px;
  clear: both;
}

div#c a {
  text-decoration: none;
  font-weight: bold;
}

div#a {
  float: left;
  width: 470px;
  padding: 3px;
  text-align: justify;
}

div#a textarea {
  width: 466px;
  height: 163px;
}

div#g {
  float: right;
  width: 470px;
  padding: 3px;
  text-align: left;
  height: 235px;
  overflow: auto;
}

div#g table {
  width: 99%;
  text-align: left;
  margin: auto;
  border-collapse: collapse;
  margin-top: 7px;
}

div#g th {
  font-weight: bold;
  border-bottom: 1px solid #5c0d12;
}

div#g td {
  height: 17px;
  padding: 3px 3px 3px 1px;
}

div#g input {
  width: 60px;
  font-size: 8pt;
}

div#h {
  float: right;
  width: 470px;
  padding: 3px;
  text-align: left;
  height: 180px;
  overflow: auto;
  padding-top: 7px;
}

div#h table {
  width: 99%;
  text-align: left;
  margin: auto;
  border-collapse: collapse;
}

div#h th {
  font-weight: bold;
  border-bottom: 1px solid #5c0d12;
}

div#h td {
  padding: 3px 3px 3px 1px;
}

div#n {
  padding: 3px;
  margin: auto;
  text-align: center;
}

div#n input {
  margin: 3px 5px;
}

td.l {
  font-weight: bold;
  vertical-align: top;
  width: 115px;
  padding: 3px;
}

td.r {
  width: 345px;
  padding: 3px;
}

td.bcn {
  font-style: italic;
  text-align: center;
  padding-top: 10px;
}

td.c1 {
  border-bottom: 0;
  border-right: 0;
}

td.c2 {
  border-right: 0;
  border-bottom: 0;
  border-left: 0;
}

td.c3 {
  border-bottom: 0;
  border-left: 0;
}

td.c4 {
  border-top: 0;
}
</style>
