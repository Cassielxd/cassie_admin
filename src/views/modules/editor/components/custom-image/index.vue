<template>
  <div class="block">
    <el-image :src="src" style="width: 100%; height: 100%" >
    </el-image>
  </div>
</template>

<script>
export default {
  props: {
    value: String,
    styles: Object,
  },
  data(){
    return {
      src: this.value
    };
  },
  methods: {
  }
}
</script>

<style lang="less" scoped>

</style>
