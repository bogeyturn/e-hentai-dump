<script setup lang="ts">
import { bountyNavItems } from "~/utils/config";
import { withDefaults } from "~/composables/useTypedQuery";
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";

const data = useTypedQuery<{
  search: string;
  p: number;
  t: "s" | "t" | "e" | "";
  s: "r" | "c" | "d" | "m1" | "m2" | "m3" | "m4" | "m5" | "m6" | "";
  u?: number;
  act?: string;
  bid?: number;
}>({
  search: withDefaults(string(), ""),
  p: withDefaults(number(), 0),
  t: withDefaults(enumOf(["s", "t", "e", ""] as const), ""),
  s: withDefaults(
    enumOf(["r", "c", "d", "m1", "m2", "m3", "m4", "m5", "m6", ""] as const),
    "",
  ),
  u: optional(number()),
  act: optional(string()),
  bid: optional(number()),
});

function title(s?: string) {
  if (s == "tops") {
    return "Most Wanted Standard Bounties";
  } else if (s == "topt") {
    return "Most Wanted Translation Bounties";
  } else if (s == "tope") {
    return "Most Wanted Editing Bounties";
  } else {
    return "Bounty List";
  }
}
</script>
<template>
  <div>
    <NavBar />
    <SubNavBar
      :nav-items="bountyNavItems"
      :bold="data.data?.bid ? '' : title(data.data?.act)"
    />
    <div v-if="data.error">{{ data.error }}</div>
    <BountyInfoWrapper
      v-else-if="data.data?.bid"
      :bid="BigInt(data.data?.bid)"
    />
    <div v-else-if="data.data?.act == 'tops'">tops</div>
    <div v-else-if="data.data?.act == 'topt'">topt</div>
    <div v-else-if="data.data?.act == 'tope'">tops</div>
    <BountyList
      v-else-if="data.data"
      :user="data.data.u"
      :bstatus="data.data.s"
      :btype="data.data.t"
      :page="data.data.p + 1"
      :search="data.data.search"
    />
  </div>
</template>
