<script setup lang="ts">

</script>

<template>
  <div class="stuffbox" style="width:584px; height:350px; margin:2px auto; text-align:left; padding:8px; font-size:9pt">
    <form id="galpop" action="" method="post">
      <p style="text-align:center">Choose a favorite category to save this gallery under. You can also add a note to it
        if you wish.</p>
      <div style="margin:8px auto 5px">
        <div class="nosel" style="float:left; width:240px">

          <div style="height:25px; cursor:pointer">
            <div style="float:left"><input
                id="fav0" type="radio" name="favcat" value="0"
                style="position:relative; top:-1px" checked="checked"
                onclick="clicked_fav(this)"/></div>
            <div
                style="float:left; padding:1px 1px 0 4px; height:15px; width:15px; background-repeat:no-repeat; background-image:url(https://ehgt.org/g/fav.png); background-position:0px -1px"
                onclick="document.getElementById('fav0').click()"/>
            <div style="float:left; padding-top:1px" onclick="document.getElementById('fav0').click()">Favorites 0</div>
            <div class="c"/>
          </div>

        </div>
        <div style="float:left; width:340px; text-align:center">
          <p style="font-weight:bold">Favorite Note (Max 200 Characters)</p>
          <textarea name="favnote" cols="50" rows="4" style="width:325px; height:160px; font-size:9pt"/><br/>
          <div style="margin:20px auto; text-align:center">
            0 / 1000 favorite note slots used. [<a href="https://ehwiki.org/wiki/Favorite" target="ehwiki">?</a>]<br/>
            <input type="submit" name="apply" value="Add to Favorites"/>
            <input type="hidden" name="update" value="1"/>
          </div>
        </div>
      </div>

    </form>
  </div>
</template>

<style scoped>

</style>