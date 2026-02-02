<script setup lang="ts">
const route = useRoute()
</script>

<template>
  <p style="padding:0 0 10px; font-size:10pt">[
    <NuxtLink
        style="text-decoration:none; font-weight:bold"
        :to="{query: {
            ...route.query, act:undefined}}">Back to Gallery
    </NuxtLink>
    ]
  </p>
  <div id="gdt" class="exp_outer">
    <div class="exp_entry">
      <form id="expunge_new">
        <h3>Submit New Expunge Petition</h3>

        <p>Specify a valid objective reason why this gallery should be expunged.</p>
        <table>
          <tbody>
          <tr>
            <td style="vertical-align:top"><label class="lr"><input
                type="radio" name="expunge_type"
                value="3"/><span/> This gallery is a duplicate
              of equal or lower quality of an earlier posted gallery.</label></td>
          </tr>
          <tr>
            <td style="vertical-align:top"><label class="lr"><input
                type="radio" name="expunge_type"
                value="6"/><span/> A newer higher-quality and
              clearly marked copy of this gallery has been uploaded.</label></td>
          </tr>
          <tr>
            <td style="vertical-align:top"><label class="lr"><input
                type="radio" name="expunge_type"
                value="5"/><span/> This gallery contains either
              illicit content like child porn or anything else that has been banned.</label></td>
          </tr>
          </tbody>
        </table>

        <div><textarea
            name="expunge_comment" cols="75" rows="5"
            placeholder="Enter a reason for this expunge here. Note that submitting petitions with subjective reasons along the line of &quot;I hate this content/artist/uploader/etc&quot; are NOT valid and can cause account penalties/restrictions."></textarea>
        </div>
        <div><input
            id="expunge_refer" type="text" name="expunge_refer"
            placeholder="Enter the URL of the conflicting gallery, if applicable." value="" style="width:554px"
            maxlength="200"/></div>
        <div><input type="submit" name="do_expunge_create" value="Create New Petition"/></div>

      </form>
    </div>
  </div>
</template>

<style scoped>
.exp_outer {
  font-size: 10pt;
  text-align: center !important;
  margin: 0 auto 10px !important
}

.exp_outer a {
  font-weight: bold
}

.exp_outer p {
  margin: 3px auto
}

.exp_outer h2 {
  margin: 3px auto 4px;
  font-size: 10pt;
  font-weight: bold
}

.exp_outer h3 {
  margin: 3px auto 4px;
  font-size: 10pt;
  font-weight: bold
}

.exp_outer textarea {
  margin: 3px auto;
  font-size: 9pt;
  width: 550px;
  padding: 3px 5px
}

.exp_entry {
  width: 700px;
  border: 1px solid #8d8d8d;
  border-radius: 9px;
  padding: 6px;
  margin: 10px auto
}

.exp_revoked {
  opacity: 0.5
}

.exp_table {
  width: 600px;
  margin: auto;
  text-align: left;
  border-top: 1px solid #34353b;
  border-bottom: 1px solid #34353b
}

.exp_table td {
  padding: 4px
}

.exp_table td:nth-child(1) {
  text-align: right;
  font-weight: bold;
  padding-right: 10px;
  white-space: nowrap;
  width: 122px
}

.exp_votelist {
  margin: 3px auto 0;
  max-width: 600px
}

.exp_votelist td {
  padding-top: 4px;
  padding-bottom: 4px;
  vertical-align: top
}

.exp_votelist td:nth-child(1) {
  width: 40px;
  text-align: right;
  font-weight: bold;
  white-space: nowrap
}

.exp_votelist td:nth-child(2) {
  text-align: left
}

.exp_votelist td:nth-child(2) > div:nth-child(1) {
  white-space: nowrap
}

.exp_votelist td:nth-child(2) > div:nth-child(2) {
  font-style: italic;
  max-width: 400px;
  overflow-wrap: break-word;
  padding: 2px 1px
}

.exp_newvote input[type="text"] {
  width: 400px
}

.exp_error {
  font-weight: bold;
  color: red
}

#expunge_new > table {
  margin: 0 auto;
  width: 560px
}

#expunge_new > table td {
  text-align: left !important;
  padding: 3px
}

#expunge_new input[type="submit"] {
  margin: 6px auto 2px
}

#revoke_new {
  margin-top: 5px
}
</style>