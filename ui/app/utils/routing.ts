import type { Router } from "vue-router";

export function updateUrl(
  router: Router,
  params: {
    f_search?: string;
    prev?: number | null;
    next?: number | null;
    cat?: number | null;
    minRating?: string | null;
    maxPages?: string | null;
    minPages?: string | null;
    disableTags?: boolean | null;
    disableUploader?: boolean | null;
    disableLang?: boolean | null;
    requireTorrent?: boolean | null;
    browseExpunged?: boolean | null;
  },
) {
  router.replace({
    query: {
      ...useRoute().query,
      ...(params.f_search !== undefined ? { f_search: params.f_search } : {}),
      ...(params.prev !== undefined
        ? params.prev !== null
          ? { prev: params.prev }
          : { prev: undefined }
        : {}),
      ...(params.next !== undefined
        ? params.next !== null
          ? { next: params.next }
          : { next: undefined }
        : {}),
      ...(params.cat !== undefined
        ? params.cat !== null
          ? { f_cats: params.cat }
          : { f_cats: undefined }
        : {}),
      ...(params.minRating !== undefined
        ? params.minRating !== null
          ? { f_srdd: params.minRating }
          : { f_srdd: undefined }
        : {}),
      ...(params.maxPages !== undefined
        ? params.maxPages !== null
          ? { f_spt: params.maxPages }
          : { f_spt: undefined }
        : {}),
      ...(params.minPages !== undefined
        ? params.minPages !== null
          ? { f_spf: params.minPages }
          : { f_spf: undefined }
        : {}),
      ...(params.disableTags !== undefined
        ? params.disableTags !== null
          ? { f_sft: 1 }
          : { f_sft: undefined }
        : {}),
      ...(params.disableUploader !== undefined
        ? params.disableUploader !== null
          ? { f_sfu: 1 }
          : { f_sfu: undefined }
        : {}),
      ...(params.disableLang !== undefined
        ? params.disableLang !== null
          ? { f_sfl: 1 }
          : { f_sfl: undefined }
        : {}),
      ...(params.requireTorrent !== undefined
        ? params.requireTorrent !== null
          ? { f_sto: 1 }
          : { f_sto: undefined }
        : {}),
      ...(params.browseExpunged !== undefined
        ? params.browseExpunged !== null
          ? { f_sh: 1 }
          : { f_sh: undefined }
        : {}),
    },
  });
}
