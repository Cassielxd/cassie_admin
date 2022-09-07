<template>
  <div
    class="line"
  >
  </div>
</template>

<script>
export default {
  props: {
    value: String,
    styles: Object,
  },
}
</script>

<style scoped>
.line{
  border-right: 1px solid black;
}
</style>
