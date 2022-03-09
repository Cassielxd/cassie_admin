<template>
  <el-dialog :visible.sync="visible" title="Excel导入" :close-on-click-modal="false" :close-on-press-escape="false">
    <el-upload
        name="file"
        :action="url"
        :file-list="fileList"
        drag
        :before-upload="beforeUploadHandle"
        :on-success="successHandle"
        class="text-center">
      <i class="el-icon-upload"></i>
      <div class="el-upload__text" v-html="$t('upload.text')"></div>
    </el-upload>
  </el-dialog>
</template>

<script>
import Cookies from 'js-cookie'
export default {
  data () {
    return {
      visible: false,
      url: '',
      fileList: []
    }
  },
  methods: {
    init () {
      this.visible = true
      this.url = `${window.SITE_CONFIG['apiURL']}/sys/excel/import?access_token=${Cookies.get('access_token')}`
      this.fileList = []
    },
    // 上传之前
    beforeUploadHandle (file) {

    },
    // 上传成功
    successHandle (res, file, fileList) {
      if (res.code !== 0) {
        return this.$message.error(res.msg)
      }
      this.$message({
        message: this.$t('prompt.success'),
        type: 'success',
        duration: 500,
        onClose: () => {
          this.visible = false
          this.$emit('refreshDataList')
        }
      })
    }
  }
}
</script>
