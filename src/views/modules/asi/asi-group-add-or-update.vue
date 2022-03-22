<template>
  <el-dialog :visible.sync="visible" :title="!dataForm.id ? $t('add') : $t('update')" :close-on-click-modal="false"
             :close-on-press-escape="false">
    <el-form :model="dataForm" :rules="dataRule" ref="dataForm" @keyup.enter.native="dataFormSubmitHandle()"
             label-width="120px">
      <el-form-item prop="group_type" label="类型" size="mini">
        <el-radio-group v-model="dataForm.group_type" :disabled="!!dataForm.id">
          <el-radio label="TABLE">TABLE</el-radio>
          <el-radio label="FROM">FROM</el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item prop="parent_name" label="父分组" class="menu-list">
        <el-popover v-model="groupListVisible" ref="menuListPopover" placement="bottom-start" trigger="click">
          <el-tree
              :data="groupList"
              :props="{ label: 'name', children: 'children' }"
              node-key="group_code"
              ref="groupListTree"
              :highlight-current="true"
              :expand-on-click-node="false"
              accordion
              @current-change="groupListTreeCurrentChangeHandle">
          </el-tree>
        </el-popover>
        <el-input v-model="dataForm.parent_name" v-popover:menuListPopover :readonly="true" placeholder="顶级父分组">
          <i v-if="dataForm.parent_group_code !== '0'" @click.stop="groupListListTreeSetDefaultHandle()" slot="suffix"
             class="el-icon-circle-close el-input__icon"></i>
        </el-input>
      </el-form-item>
      <el-form-item prop="name" label="分组名称">
        <el-input v-model="dataForm.name" placeholder="业务分组名称"></el-input>
      </el-form-item>
      <el-form-item prop="info" label="描述">
        <el-input v-model="dataForm.info" placeholder="分组描述"></el-input>
      </el-form-item>
      <el-form-item prop="group_code" label="分组编码">
        <el-input v-model="dataForm.group_code" placeholder="分组编码唯一"></el-input>
      </el-form-item>

    </el-form>
    <template slot="footer">
      <el-button @click="visible = false">{{ $t('cancel') }}</el-button>
      <el-button type="primary" @click="dataFormSubmitHandle()">{{ $t('confirm') }}</el-button>
    </template>
  </el-dialog>
</template>

<script>
import debounce from 'lodash/debounce'

export default {
  data () {
    return {
      visible: false,
      groupListVisible: false,
      groupList: [],
      dataForm: {
        id: '',
        name: '',
        info: '',
        group_code: '',
        parent_group_code: '',
        parent_name: '',
        group_type: ''
      }
    }
  },
  computed: {
    dataRule () {
      return {
        name: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        info: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ],
        group_code: [
          { required: true, message: this.$t('validate.required'), trigger: 'blur' }
        ]
      }
    }
  },
  methods: {
    init () {
      this.visible = true
      this.$nextTick(() => {
        this.$refs['dataForm'].resetFields()
        this.getGroupList().then(() => {
          if (this.dataForm.id) {
            this.getInfo()
            this.formatList(this.dataForm.parent_group_code)
          }
        })
      })
    },
    getGroupList () {
      return this.$http.get('/asi/group/list?parent_group_code=0').then(({ data: res }) => {
        // eslint-disable-next-line eqeqeq
        if (res.code != 0) {
          return this.$message.error(res.msg)
        }
        this.groupList = res.data
      }).catch(() => {
      })
    },
    groupListListTreeSetDefaultHandle () {
      this.dataForm.parent_group_code = '0'
      this.dataForm.parent_name = '顶级分组'
    },
    groupListTreeCurrentChangeHandle (data) {
      this.dataForm.parent_group_code = data.group_code
      this.dataForm.parent_name = data.name
      this.groupListVisible = false
    },
    // 获取信息
    getInfo () {
      this.$http.get(`/asi/group/${this.dataForm.id}`).then(({ data: res }) => {
        // eslint-disable-next-line eqeqeq
        if (res.code != 0) {
          return this.$message.error(res.msg)
        }
        this.dataForm = {
          ...this.dataForm,
          ...res.data
        }
      }).catch(() => {
      })
    },
    // eslint-disable-next-line camelcase
    formatList (groupCode) {
      let list = this.groupList
      // eslint-disable-next-line eqeqeq
      let f = list.filter(v => groupCode == v.group_code)
      if (f.length > 0) {
        this.dataForm.parent_name = f[0].name
        this.$refs.groupListTree.setCurrentKey(groupCode)
      }
    },
    // 表单提交
    dataFormSubmitHandle: debounce(function () {
      this.$refs['dataForm'].validate((valid) => {
        if (!valid) {
          return false
        }

        this.$http[!this.dataForm.id ? 'post' : 'put']('/asi/group', this.dataForm).then(({ data: res }) => {
          if (res.code != 0) {
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
        }).catch(() => {
        })
      })
    }, 1000, { 'leading': true, 'trailing': false })
  }
}
</script>
