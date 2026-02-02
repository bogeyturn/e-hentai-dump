<script setup lang="ts">
const route = useRoute()
const router = useRouter()
const report = computed<string>({
  get() {
    return String(route.query.report ?? "select");
  },
  set(value: string) {
    router.replace({
      query: {
        ...route.query,
        report: value,
      },
    });
  },
})
</script>

<template>
  <p style="padding:0 0 10px; font-size:10pt">[
    <NuxtLink
        style="text-decoration:none; font-weight:bold"
        :to="{query: {
            ...route.query, report:undefined}}">Back to Gallery
    </NuxtLink>
    ]
  </p>
  <div id="gdt" style="font-size:10pt; text-align:center">
    <form id="reportform" @submit.prevent="">
      <table id="reporttable" style="margin:auto; width:936px">
        <tbody>
        <tr>
          <td style="text-align:right; width:200px; padding:4px"><strong>Report Type</strong></td>
          <td style="width:510px"><select
              id="report_type"
              v-model="report" name="report_type"
              style="width:500px; padding:4px">
            <option value="select">[Select a complaint type...]</option>
            <optgroup label="Content">
              <option value="dmca">DMCA/Copyright Infringement</option>
              <option value="child">Child Pornography</option>
              <option value="other">Other Illicit Content</option>
            </optgroup>
            <optgroup label="Technical">
              <option value="excupd">Excessive Updates</option>
              <option value="revert">Reversion/Split/Undisown Request</option>
            </optgroup>
          </select></td>
          <td style="width:200px"/>
        </tr>
        <template v-if="report == 'dmca'">
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Your email address</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_email" type="text" name="r_email"
                placeholder="Please provide a working email address."
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Your full legal name</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_fullname" type="text"
                name="r_fullname"
                placeholder="Aliases and pseudonyms cannot be used."
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Copyright holder's full name</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_copyname" type="text"
                name="r_copyname"
                placeholder="Enter if you are not the copyright holder. You must be an authorized agent."
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Company Name</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_compname" type="text"
                name="r_compname" placeholder="Optional"
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Phone Number</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_phone" type="text" name="r_phone"
                placeholder="Optional" value=""
                style="width:500px" maxlength="200"/>
            </td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Address</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_address" type="text"
                name="r_address"
                placeholder="Your residential home address"
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">City</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_city" type="text" name="r_city"
                placeholder="Your city of residence"
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">State / Province</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_state" type="text" name="r_state"
                placeholder="Your state or province of residence, if applicable"
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Country</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_country" type="text"
                name="r_country"
                placeholder="Your country of residence"
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Describe the nature of the
              infringement</p>
              <p style="font-size:-1">Elaborate on why this gallery is infringing on your copyrighted work. Include
                links to where the original work can be purchased, or evidence of source material.</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><textarea
                id="r_details" name="r_details"
                placeholder="Please elaborate on why this gallery infringes on your copyright. If this work is for sale, provide links to where it can be legally purchased."
                style="width:500px; height:200px"/>
            </td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Digital signature</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_digisign" type="text"
                name="r_digisign"
                placeholder="Enter your name to sign this submission"
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Acknowledgement</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><p>By submitting this form, you attest, under
              penalty of perjury, that you have a good faith belief that use of the material in this report is not
              authorized by the copyright owner, its agent, or the law; AND you are authorized to act on behalf of the
              copyright owner; AND you understand that you may be liable for any damages, including costs and attorneys'
              fees, if you knowingly materially misrepresent reported material.</p></td>
          </tr>
        </template>
        <template v-else-if="report != 'select'">
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Your email address</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><input
                id="r_email" type="text" name="r_email"
                placeholder="Please provide a working email address."
                value="" style="width:500px"
                maxlength="200"/></td>
          </tr>
          <tr>
            <td style="text-align:right; padding:4px"><p style="font-weight:bold">Report Details</p>
              <p style="font-size:-1">Please describe the illicit content of this gallery. Include the specific pages
                where the content in question can be found.</p></td>
            <td style="vertical-align:top; text-align:left; padding:4px"><textarea
                id="r_details" name="r_details"
                placeholder="Details of illicit content and what pages you found it on. Using this form for copyright-related requests or anything else that is not actual illicit content can get you blocked from using this site."
                style="width:500px; height:200px"/>
            </td>
          </tr>
        </template>

        <tr v-if="report != 'select'">
          <td/>
          <td><input type="submit" name="sendreport" value="Send Report" style="width:120px; height:30px"/></td>
        </tr>
        </tbody>
      </table>
    </form>
  </div>
</template>

<style scoped>

</style>