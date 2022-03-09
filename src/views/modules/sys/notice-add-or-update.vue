<template>
  <el-dialog :visible.sync="visible" :title="!dataForm.id ? $t('add') : $t('update')" :close-on-click-modal="false" :close-on-press-escape="false">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" :label-width="$i18n.locale === 'en-US' ? '120px' : '80px'">
      <el-form-item :label="$t('notice.type')" prop="type">
          <ren-radio-group v-model="dataForm.type" dict-type="notice_type"></ren-radio-group>
      </el-form-item>
      <el-form-item :label="$t('notice.title')"  prop="title">
          <el-input v-model="dataForm.title" :placeholder="$t('notice.title')"></el-input>
      </el-form-item>
      <el-form-item prop="content" :label="$t('notice.content')">
        <!-- 富文本编辑器, 容器 -->
        <div id="J_quillEditor" style="height:280px"></div>
        <!-- 自定义上传图片功能 (使用element upload组件) -->
        <el-upload
                :action="uploadUrl"
                :show-file-list="false"
                :before-upload="uploadBeforeUploadHandle"
                :on-success="uploadSuccessHandle"
                style="display: none;">
            <el-button ref="uploadBtn" type="primary" size="small">{{ $t('upload.button') }}</el-button>
        </el-upload>
      </el-form-item>
      <el-form-item :label="$t('notice.receiverType')" prop="">
          <el-radio-group v-model="dataForm.receiverType">
              <el-radio :label="0">{{ $t('notice.receiverType0') }}</el-radio>
              <el-radio :label="1">{{ $t('notice.receiverType1') }}</el-radio>
          </el-radio-group>
      </el-form-item>
      <el-form-item v-show="dataForm.receiverType == 1" size="mini" :label="$t('notice.selectDept')">
        <el-tree
            :data="deptList"
            :props="{ label: 'name', children: 'children' }"
            node-key="id"
            ref="deptListTree"
            accordion
            show-checkbox>
        </el-tree>
      </el-form-item>
    </el-form>
    <template slot="footer">
      <el-button @click="visible = false">{{ $t('cancel') }}</el-button>
      <el-button type="danger" @click="dataFormSubmitHandle(0)">{{ $t('notice.draft') }}</el-button>
      <el-button type="primary" @click="dataFormSubmitHandle(1)">{{ $t('notice.release') }}</el-button>
    </template>
  </el-dialog>
</template>

<script>
import Cookies from 'js-cookie'
import debounce from 'lodash/debounce'
import 'quill/dist/quill.snow.css'
import Quill from 'quill'
export default {
  data () {
    return {
      visible: false,
      quillEditor: null,
      quillEditorToolbarOptions: [
        ['bold', 'italic', 'underline', 'strike'],
        ['image'],
        [{ 'list': 'ordered' }, { 'list': 'bullet' }],
        [{ 'size': ['small', false, 'large', 'huge'] }],
        [{ 'color': [] }, { 'background': [] }],
        ['clean']
      ],
      uploadUrl: '',
      deptList: [],
      dataForm: {
        id: '',
        type: 0,
        title: '',
        content: '',
        receiverType: 0,
        receiverTypeIds: '',
        receiverTypeList: [],
        status: '',
        senderName: '',
        senderDate: '',
        creator: '',
        createDate: ''
      }
    }
  },
  computed: {
    dataRule () {
      var validateContent = (rule, value, callback) => {
        if (this.quillEditor.getLength() <= 1) {
          return callback(new Error(this.$t('validate.required')))
        }
        callback()
      }
      return {
        type: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        title: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        content: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' },
          { validator: validateContent, trigger: 'blur' }
        ],
        receiverType: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        receiverTypeIds: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        status: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        senderName: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ]
      }
    }
  },
  methods: {
    init () {
      this.visible = true
      this.$nextTick(() => {
        if (this.quillEditor) {
          this.quillEditor.deleteText(0, this.quillEditor.getLength())
        } else {
          this.quillEditorHandle()
        }
        this.$refs['dataForm'].resetFields()
        Promise.all([
          this.getDeptList()
        ]).then(() => {
          if (this.dataForm.id) {
            this.getInfo()
          }
        })
      })
    },
    // 富文本编辑器
    quillEditorHandle () {
      this.quillEditor = new Quill('#J_quillEditor', {
        modules: {
          toolbar: this.quillEditorToolbarOptions
        },
        theme: 'snow'
      })
      // 自定义上传图片功能 (使用element upload组件)
      this.uploadUrl = `${window.SITE_CONFIG['apiURL']}/sys/oss/upload?access_token=${Cookies.get('access_token')}`
      this.quillEditor.getModule('toolbar').addHandler('image', () => {
        this.$refs.uploadBtn.$el.click()
      })
      // 监听内容变化，动态赋值
      this.quillEditor.on('text-change', () => {
        this.dataForm.content = this.quillEditor.root.innerHTML
      })
    },
    // 上传图片之前
    uploadBeforeUploadHandle (file) {
      if (file.type !== 'image/jpg' && file.type !== 'image/jpeg' && file.type !== 'image/png' && file.type !== 'image/gif') {
        this.$message.error(this.$t('upload.tip', { 'format': 'jpg、png、gif' }))
        return false
      }
    },
    // 上传图片成功
    uploadSuccessHandle (res, file, fileList) {
      if (res.code !== 0) {
        return this.$message.error(res.msg)
      }
      this.quillEditor.insertEmbed(this.quillEditor.getSelection().index, 'image', res.data.src)
    },
    // 获取部门列表
    getDeptList () {
      return this.$http.get('/sys/dept/list').then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.deptList = res.data
      }).catch(() => {})
    },
    // 获取信息
    getInfo () {
      this.$http.get(`/sys/notice/${this.dataForm.id}`).then(({ data: res }) => {
        if (res.code !== 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }

        this.quillEditor.root.innerHTML = this.dataForm.content

        // 接受者为部门
        if (this.dataForm.receiverType === 1) {
          this.$refs.deptListTree.setCheckedKeys(res.data.receiverTypeIds.split(','))
        }
      }).catch(() => {})
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function (status) {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }
        // 接受者为部门
        if (this.dataForm.receiverType === 1) {
          this.dataForm.receiverTypeIds = this.$refs.deptListTree.getCheckedKeys().join(',')
          this.dataForm.receiverTypeList = this.$refs.deptListTree.getCheckedKeys()
        } else {
          this.dataForm.receiverTypeIds = ''
        }
        this.dataForm.status = status
        this.$http[!this.dataForm.id ? 'post' : 'put']('/sys/notice/', this.dataForm).then(({ data: res }) => {
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
        }).catch(() => {})
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>
