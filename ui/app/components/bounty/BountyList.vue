<script setup lang="ts">
const ses = getSession();

const router = useRouter();

const props = defineProps<{
  search: string;
  page: number;
  btype: "s" | "t" | "e" | "";
  bstatus: "r" | "c" | "d" | "m1" | "m2" | "m3" | "m4" | "m5" | "m6" | "";
  user: number | undefined;
}>();

const btypeDisplay = () => {
  switch (props.btype) {
    case "":
      return "All";
    case "e":
      return "Editing";
    case "s":
      return "Standard";
    default:
      return "Translation";
  }
};

const bstatusDisplay = () => {
  switch (props.bstatus) {
    case "":
      return "Open";
    case "r":
      return "Reserved";
    case "c":
      return "Claimed";
    case "d":
      return "Completed";
    case "m1":
      return "PostedMe";
    case "m2":
      return "BoostedMe";
    case "m3":
      return "AcceptedMe";
    case "m4":
      return "ClaimedMe";
    case "m5":
      return "CompletedMe";
    case "m6":
      return "ReservedMe";
  }
};

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () =>
    `bountyList-${props.btype}-${props.page}-${props.bstatus}-${props.user}-${props.search}`,
  async () => {
    try {
      const v = await ses.bounty(
        props.search,
        props.page,
        btypeDisplay(),
        bstatusDisplay(),
        props.user,
      );
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

const btypeModel = computed({
  get: () => props.btype,
  set: (value) => {
    router.replace({
      query: {
        ...route.query,
        t: value || undefined,
      },
    });
  },
});

const statusModel = computed({
  get: () => props.bstatus,
  set: (value) => {
    router.replace({
      query: {
        ...route.query,
        s: value || undefined,
      },
    });
  },
});

const query = ref("");

const route = useRoute();
</script>

<template>
  <div v-if="data && !('error' in data)" id="s" class="stuffbox">
    <h1>Showing All Open Bounties</h1>
    <div>
      <form id="searchform">
        <div>
          <input
            id="s"
            v-model="query"
            type="text"
            name="search"
            placeholder="Search Keywords"
            size="50"
            maxlength="255"
          />
          <NuxtLink
            :to="{
              path: route.path,
              query: { ...route.query, p: undefined, search: query },
            }"
            ><input type="button" name="dc1" value="Search Bounties"
          /></NuxtLink>
          <NuxtLink
            :to="{
              path: route.path,
              query: {},
            }"
            ><input type="button" name="dc2" value="Clear"
          /></NuxtLink>
        </div>
        <div>
          Bounty Type:
          <select v-model="btypeModel" name="t" style="width: 100px">
            <option value="">All</option>
            <option value="s">Standard</option>
            <option value="t">Translation</option>
            <option value="e">Editing</option>
          </select>
          &nbsp; Bounty Status:
          <select v-model="statusModel" name="s" style="width: 200px">
            <option value="">All Open Bounties</option>
            <option value="r">All Reserved Bounties</option>
            <option value="c">All Claimed Bounties</option>
            <option value="d">All Completed Bounties</option>
            <option value="m1">Bounties Posted By Me</option>
            <option value="m2">Bounties Boosted By Me</option>
            <option value="m3">Bounties Accepted By Me</option>
            <option value="m6">Bounties Reserved For Me</option>
            <option value="m4">Bounties Claimed By Me</option>
            <option value="m5">Bounties Completed By Me</option>
          </select>
        </div>
      </form>
    </div>
    <ImagePagination
      :page-size="1"
      :current-page="page"
      :total-items="data.pages"
      :top="true"
      :show-counter="false"
    />
    <table class="itg">
      <tbody>
        <tr>
          <th style="width: 99px">Last Updated</th>
          <th>Bounty Headline</th>
          <th style="width: 80px">Bounty Type</th>
          <th style="width: 100px">Bounty Status</th>
          <th style="width: 180px">Total Bounty</th>
          <th style="width: 130px">Posted By</th>
        </tr>

        <tr v-for="(item, i) in data.items" :key="i">
          <td class="itd">{{ displayDate(item.last_updated) }}</td>
          <td class="itd">
            <NuxtLink :to="`/bounty?bid=${item.id}`"
              >{{ item.headline }}
            </NuxtLink>
          </td>
          <td class="itd">{{ item.bounty_type }}</td>
          <td class="itd">
            <span
              :class="{
                scr: item.status == 'Closed/Completed',
                sco:
                  item.status == 'Closed/Claimed' ||
                  item.status == 'Closed/Reserved',
                scb: item.status == 'Open/Accepted',
                scg: item.status == 'Open/New',
              }"
              >{{ item.status }}</span
            >
          </td>
          <td class="itd">{{ item.total }}</td>
          <td class="itd">
            <NuxtLink :to="`/bounty?u=${item.uid}`"
              >{{ item.posted_by }}
            </NuxtLink>
          </td>
        </tr>
      </tbody>
    </table>
    <ImagePagination
      :page-size="1"
      :current-page="page"
      :total-items="data.pages"
      :top="false"
      :show-counter="false"
    />
    <div id="r">{{ data.msg }}</div>
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

div#s {
  width: 1200px;
  margin: 10px auto 10px;
  padding: 3px;
}

div#s h1 {
  font-weight: bold;
  font-size: 10pt;
}

div#s div {
  margin: 1px auto 6px;
}

input#s {
  font-size: 10pt;
  width: 500px;
}

div#r {
  margin: auto;
  font-weight: bold;
  width: 300px;
  text-align: center;
}
</style>
