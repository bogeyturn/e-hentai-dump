<script setup lang="ts">
import { configNavItems } from "~/utils/config";
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";
import { formatBytesIB } from "~/utils/size";

const ses = getSession();

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `h@h`,
  async () => {
    try {
      const v = await ses.home();
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);
</script>
<template>
  <div>
    <NavBar />
    <SubNavBar :nav-items="configNavItems" bold="Overview" />
    <div v-if="data && !('error' in data)" class="stuffbox">
      <h2>Image Limits</h2>

      <div class="homebox">
        <p>
          Due to widespread usage of bulk downloaders, high-resolution images
          can be limited.
        </p>
        <p>
          You are currently using IP-based limits. No restrictions are currently
          in effect.
        </p>
        <p>
          You can get a Bronze Star or the More Pages hathperk to tie image
          limits to your account.
        </p>
        <p>
          Alternatively, you can unlock a high-resolution quota for 24 hours by
          spending <strong>20,000</strong> GP.
        </p>
        <p>
          Note that for the latter, clearing your cookies will revert you to
          IP-based limits.
        </p>

        <form @submit.prevent="">
          <p>
            <input
              type="submit"
              name="reset_imagelimit"
              value="Unlock Quota"
              style="width: 120px"
            />
          </p>
        </form>
      </div>

      <h2>EHTracker</h2>

      <div class="homebox">
        <table style="margin: 10px auto">
          <tbody>
            <tr>
              <td class="c1">
                {{ formatBytesIB(data.ehtracker.uploaded_bytes) }}
              </td>
              <td class="c2">uploaded</td>
              <td class="c1">
                {{ formatBytesIB(data.ehtracker.downloaded_bytes) }}
              </td>
              <td class="c2">downloaded</td>
              <td class="c1" style="color: green">
                {{ data.ehtracker.up_down_ratio }}
              </td>
              <td class="c2">up/down ratio</td>
            </tr>
            <tr>
              <td class="c1">{{ data.ehtracker.torrent_completes }}</td>
              <td class="c2">torrent completes</td>
              <td class="c1">{{ data.ehtracker.gallery_completes }}</td>
              <td class="c2">gallery completes</td>
              <td class="c1">
                {{ data.ehtracker.seedmins.toLocaleString("en-US") }}
              </td>
              <td class="c2">seedmins</td>
            </tr>
          </tbody>
        </table>

        <div style="margin: 10px auto">
          <NuxtLink :to="`/torrents?u=${data.user_id}`"
            >Show My Torrents
          </NuxtLink>
        </div>

        <p>
          If you misplace any of your personalized torrents, hit the button
          below to reset your key.<br />This will immediately invalidate all of
          your personalized torrents in play.
        </p>
        <p>
          Your current key is:
          <span style="font-weight: bold">{{ data.ehtracker.key }}</span>
        </p>

        <form @submit.prevent="">
          <p>
            <input
              type="submit"
              name="reset_torrent_key"
              value="Reset Torrent Key"
              style="width: 120px; padding-bottom: 3px"
            />
          </p>
        </form>
      </div>

      <h2>Total GP Gained</h2>

      <div class="homebox">
        <table style="margin: auto; text-align: left">
          <tbody>
            <tr>
              <td style="font-weight: bold; text-align: right">
                {{ data.gp.gallery_visits.toLocaleString("en-US") }}
              </td>
              <td>GP from gallery visits</td>
            </tr>
            <tr>
              <td style="font-weight: bold; text-align: right">
                {{ data.gp.torrent_completions.toLocaleString("en-US") }}
              </td>
              <td>GP from torrent completions</td>
            </tr>
            <tr>
              <td style="font-weight: bold; text-align: right">
                {{ data.gp.archive_downloads.toLocaleString("en-US") }}
              </td>
              <td>GP from archive downloads</td>
            </tr>
            <tr>
              <td style="font-weight: bold; text-align: right">
                {{ data.gp.hentai_at_home.toLocaleString("en-US") }}
              </td>
              <td>GP from Hentai@Home</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2>Toplists</h2>

      <div class="homebox">
        <table style="text-align: left; margin: auto">
          <tbody>
            <tr v-if="data.toplist.length == 0">
              <td style="vertical-align: top; padding-top: 4px" />
              <td>You are currently not featured on any toplists...</td>
            </tr>
            <tr v-for="(item, index) in data.toplist" :key="index">
              <td style="vertical-align: top; padding-top: 4px">
                You are currently:
              </td>
              <td>
                <table>
                  <tbody>
                    <tr>
                      <td style="text-align: right">
                        <strong>#{{ item.position }}</strong>
                      </td>
                      <td>
                        on the
                        <NuxtLink
                          :to="`/toplist?tl=${item.toplist_id}`"
                          style="
                            text-decoration: none;
                            font-weight: bold;
                            white-space: nowrap;
                          "
                          >{{ item.name }}
                        </NuxtLink>
                        toplist
                      </td>
                    </tr>
                  </tbody>
                </table>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2>Moderation Power</h2>

      <div class="homebox">
        <table style="margin: auto; width: 100%">
          <tbody>
            <tr>
              <td
                style="
                  width: 50%;
                  vertical-align: top;
                  padding-top: 35px;
                  border-right: 1px solid #5c0d12;
                "
              >
                <div>Current Moderation Power</div>
                <div
                  style="margin-top: 5px; font-size: 14pt; font-weight: bold"
                >
                  {{ data.power.current }}
                </div>
              </td>
              <td>
                <table style="text-align: right; margin: auto">
                  <tbody>
                    <tr>
                      <td>Base</td>
                      <td style="font-weight: bold">+{{ data.power.base }}</td>
                    </tr>
                    <tr>
                      <td>Awards</td>
                      <td style="font-weight: bold">
                        +{{ data.power.awards }}
                      </td>
                    </tr>
                    <tr>
                      <td>Tagging</td>
                      <td style="font-weight: bold">
                        +{{ data.power.tagging }}
                      </td>
                    </tr>
                    <tr>
                      <td>&nbsp;</td>
                      <td style="font-weight: bold">
                        =
                        {{
                          data.power.base +
                          data.power.awards +
                          data.power.tagging
                        }}
                      </td>
                    </tr>
                    <tr>
                      <td>&nbsp;</td>
                      <td style="font-weight: bold">&nbsp;</td>
                    </tr>
                    <tr>
                      <td>&nbsp;</td>
                      <td style="font-weight: bold">&nbsp;</td>
                    </tr>
                  </tbody>
                </table>
              </td>
              <td>
                <table style="text-align: right; margin: auto">
                  <tbody>
                    <tr>
                      <td>Level</td>
                      <td style="font-weight: bold">+{{ data.power.level }}</td>
                    </tr>
                    <tr>
                      <td>Donations</td>
                      <td style="font-weight: bold">
                        +{{ data.power.donations }}
                      </td>
                    </tr>
                    <tr>
                      <td>Forum Activity</td>
                      <td style="font-weight: bold">
                        +{{ data.power.forum_activity }}
                      </td>
                    </tr>
                    <tr>
                      <td>Uploads/H@H</td>
                      <td style="font-weight: bold">
                        +{{ data.power.uploads }}
                      </td>
                    </tr>
                    <tr>
                      <td>Account Age</td>
                      <td style="font-weight: bold">
                        +{{ data.power.account_age }}
                      </td>
                    </tr>
                    <tr>
                      <td>(capped to 25)</td>
                      <td style="font-weight: bold">= {{ data.power.sum }}</td>
                    </tr>
                  </tbody>
                </table>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<style scoped>
h1 {
  font-size: 10pt;
  font-weight: bold;
  text-align: center;
  margin: 3px;
}

h2 {
  font-size: 10pt;
  font-weight: bold;
  text-align: center;
  margin: 10px auto 3px;
}

.stuffbox {
  min-width: 600px;
  max-width: 980px;
  margin: 10px auto;
  padding: 3px 3px 15px;
  font-size: 10pt;
}

.homebox {
  border: 1px solid #5c0d12;
  width: 600px;
  margin: auto;
  padding: 3px;
}

.c1 {
  width: 100px;
  font-weight: bold;
  text-align: right;
}

.c2 {
  width: 120px;
  text-align: left;
  margin-bottom: 2px;
}
</style>
