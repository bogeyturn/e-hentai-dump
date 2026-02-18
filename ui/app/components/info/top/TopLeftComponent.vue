<script setup lang="ts">
import Category from "~/components/views/Category.vue";
import TopRightComponent from "~/components/info/top/TopRightComponent.vue";

const props = defineProps<{
  idd: bigint;
  apiuid: bigint | null;
  token: string;
  category: string;
  uploaderId: number | null;
  parent: { id: number; key: string } | null;
  uploader: string;
  posted: { secs: number };
  visible: boolean;
  language: string;
  size: number;
  files: number;
  myStars: number | null;
  rating: number | null;
  apikey: string | null;
  favorited: number;
  favorite: number | null;
}>();

const ses = getSession();
const cookie = useCookie("EX_COOKIE");

const stars = ref(props.myStars ?? 0 / 2);
watch(
    () => props.idd,
    (_n, _o) => {
      stars.value = props.myStars ?? 0 / 2;
    },
);

const emits = defineEmits(["close"]);

async function setStars(count: number, double: boolean = false) {
  if (double) {
    stars.value = count;
    count *= 2;
  } else {
    stars.value = count / 2;
  }

  await ses.rateGallery(
      props.idd,
      props.token,
      BigInt(1),
      "",
      count,
      true
  );
  if (props.apiuid && props.apikey) {
    await ses.rateGallery(
        props.idd,
        props.token,
        props.apiuid,
        props.apikey,
        count,
        false
    );
  }

  const cookie_val = await ses.cookie();
  if (cookie_val != cookie.value) cookie.value = cookie_val;
}

function getNumberFromCode(e: KeyboardEvent): number | null {
  if (e.code.startsWith("Digit")) return parseInt(e.code.slice(5), 10);
  if (e.code.startsWith("Numpad")) {
    const num = parseInt(e.code.slice(6), 10);
    return isNaN(num) ? null : num;
  }
  return null;
}

function handleKeydown(e: KeyboardEvent) {
  const tag = (e.target as HTMLElement).tagName.toLowerCase();
  const isTyping =
      tag === "input" ||
      tag === "textarea" ||
      (e.target as HTMLElement).isContentEditable;

  const keyNum = getNumberFromCode(e);
  if (!keyNum || isTyping) return;

  if (!e.ctrlKey && !e.metaKey && !e.shiftKey) setStars(keyNum);
  if (!e.ctrlKey && !e.metaKey && e.shiftKey) {
    emits("close");
    setStars(keyNum); //window.open("", "_self").close()
  }
}

// --- Add / remove keydown listener ---
onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
});
onUnmounted(() => {
  window.removeEventListener("keydown", handleKeydown);
});
</script>

<template>
  <div id="gd3">
    <div id="gdc">
      <Category class="cs" :category="category"/>
    </div>
    <div id="gdn">
      <NuxtLink to="/">{{ uploader }}</NuxtLink>
      <template v-if="uploaderId">
        &nbsp;
        <NuxtLink to="https://forums.e-hentai.org/index.php?showuser=4269631"
        ><img class="ygm" src="/g/ygm.png" alt="PM" title="Contact Uploader"
        /></NuxtLink>
      </template>
    </div>
    <div id="gdd">
      <table>
        <tbody>
        <tr>
          <td class="gdt1">Posted:</td>
          <td class="gdt2">
            {{ displayDate(posted) }}
          </td>
        </tr>
        <tr>
          <td class="gdt1">Parent:</td>
          <td class="gdt2">
            <NuxtLink v-if="parent" :to="`/g/${parent.id}/${parent.key}`">
              {{ parent.id }}
            </NuxtLink>
            <template v-else> None</template>
          </td>
        </tr>
        <tr>
          <td class="gdt1">Visible:</td>
          <td class="gdt2">
            {{ visible ? "Yes" : "No" }}
          </td>
        </tr>
        <tr>
          <td class="gdt1">Language:</td>
          <td class="gdt2">
            <template v-if="language.endsWith('TR')">
              {{ language.substring(0, language.length - 4) }}
              &nbsp;<span
                class="halp"
                title="This gallery has been translated from the original language text."
            >TR</span
            >
            </template>
            <template v-else>
              {{ language }}
            </template>
          </td>
        </tr>
        <tr>
          <td class="gdt1">File Size:</td>
          <td class="gdt2">
            {{ (size / (1024 * 1024)).toFixed(2) }}
            MiB
          </td>
        </tr>
        <tr>
          <td class="gdt1">Length:</td>
          <td class="gdt2">{{ files }} pages</td>
        </tr>
        <tr>
          <td class="gdt1">Favorited:</td>
          <td id="favcount" class="gdt2">
            <template v-if="favorited == 1">1 time</template>
            <template v-else>{{ favorited }} times</template>
          </td>
        </tr>
        </tbody>
      </table>
    </div>
    <div id="gdr">
      <table>
        <tbody>
        <tr>
          <td id="grt1" style="text-align: start; vertical-align: top">
            Rating:
          </td>
          <td id="grt2" rowspan="2">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 114 34"
                width="114"
                height="34"
            >
              <linearGradient
                  id="starGradient"
                  x1="0%"
                  y1="100%"
                  x2="0%"
                  y2="0%"
              >
                <!-- bottom color -->
                <stop offset="0%" stop-color="#ffcc65"/>
                <!-- top color -->
                <stop offset="100%" stop-color="#fff7af"/>
              </linearGradient>

              <symbol id="star" viewBox="0 0 24 24">
                <path
                    d="M9.15316 5.40838C10.4198 3.13613 11.0531 2 12 2C12.9469 2 13.5802 3.13612 14.8468 5.40837L15.1745 5.99623C15.5345 6.64193 15.7144 6.96479 15.9951 7.17781C16.2757 7.39083 16.6251 7.4699 17.3241 7.62805L17.9605 7.77203C20.4201 8.32856 21.65 8.60682 21.9426 9.54773C22.2352 10.4886 21.3968 11.4691 19.7199 13.4299L19.2861 13.9372C18.8096 14.4944 18.5713 14.773 18.4641 15.1177C18.357 15.4624 18.393 15.8341 18.465 16.5776L18.5306 17.2544C18.7841 19.8706 18.9109 21.1787 18.1449 21.7602C17.3788 22.3417 16.2273 21.8115 13.9243 20.7512L13.3285 20.4768C12.6741 20.1755 12.3469 20.0248 12 20.0248C11.6531 20.0248 11.3259 20.1755 10.6715 20.4768L10.0757 20.7512C7.77268 21.8115 6.62118 22.3417 5.85515 21.7602C5.08912 21.1787 5.21588 19.8706 5.4694 17.2544L5.53498 16.5776C5.60703 15.8341 5.64305 15.4624 5.53586 15.1177C5.42868 14.773 5.19043 14.4944 4.71392 13.9372L4.2801 13.4299C2.60325 11.4691 1.76482 10.4886 2.05742 9.54773C2.35002 8.60682 3.57986 8.32856 6.03954 7.77203L6.67589 7.62805C7.37485 7.4699 7.72433 7.39083 8.00494 7.17781C8.28555 6.96479 8.46553 6.64194 8.82547 5.99623L9.15316 5.40838Z"
                    stroke-width="1.5"
                />
              </symbol>

              <use
                  href="#star"
                  x="0"
                  y="0"
                  width="16"
                  height="16"
                  :stroke="stars >= 1.0 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 1.0 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(1.0, true)"
              />
              <use
                  href="#star"
                  x="21"
                  y="0"
                  width="16"
                  height="16"
                  :stroke="stars >= 2.0 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 2.0 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(2.0, true)"
              />
              <use
                  href="#star"
                  x="42"
                  y="0"
                  width="16"
                  height="16"
                  :stroke="stars >= 3.0 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 3.0 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(3.0, true)"
              />
              <use
                  href="#star"
                  x="63"
                  y="0"
                  width="16"
                  height="16"
                  :stroke="stars >= 4.0 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 4.0 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(4.0, true)"
              />
              <use
                  href="#star"
                  x="84"
                  y="0"
                  width="16"
                  height="16"
                  :stroke="stars >= 5.0 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 5.0 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(5.0, true)"
              />

              <use
                  href="#star"
                  x="6"
                  y="10"
                  width="24"
                  height="24"
                  :stroke="stars >= 0.5 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 0.5 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(0.5, true)"
              />
              <use
                  href="#star"
                  x="27"
                  y="10"
                  width="24"
                  height="24"
                  :stroke="stars >= 1.5 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 1.5 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(1.5, true)"
              />
              <use
                  href="#star"
                  x="48"
                  y="10"
                  width="24"
                  height="24"
                  :stroke="stars >= 2.5 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 2.5 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(2.5, true)"
              />
              <use
                  href="#star"
                  x="69"
                  y="10"
                  width="24"
                  height="24"
                  :stroke="stars >= 3.5 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 3.5 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="setStars(3.5, true)"
              />
              <use
                  href="#star"
                  x="90"
                  y="10"
                  width="24"
                  height="24"
                  :stroke="stars >= 4.5 ? '#ffcc65' : '#cecece'"
                  :fill="stars >= 4.5 ? 'url(#starGradient)' : '#f9f9f9'"
                  @click="
                    () => {
                      setStars(4.5, true);
                    }
                  "
              />
            </svg>
          </td>
        </tr>
        <tr>
          <td>{{ rating }}</td>
        </tr>
        </tbody>
      </table>
      <div style="width: 170px;display: flex;justify-content: center;">
        <TopRightComponent :idd="idd" :token="token" :favorite="favorite"
                           @close="$emit('close')"
        />
      </div>
    </div>
  </div>
</template>

<style scoped></style>
