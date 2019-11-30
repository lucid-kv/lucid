<template>
  <div>
    <h1>KV Proof of Concept</h1>

    <b-card header="Targetted key">
      <b-form-group label="Key" label-for="key">
        <b-form-input
          v-model="lucidKey"
          id="key"
          placeholder="Key"
          :disabled="isKvLoading"
          @keypress.enter="loadSelectedKey"
        />
      </b-form-group>
    </b-card>

    <transition name="fade" mode="out-in">
      <div v-if="lucidKey">
        <GetKey :lucid-key="lucidKey" ref="getKeyComponent" />
        <SetKey :lucid-key="lucidKey" />
      </div>
    </transition>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'

import GetKey from '@/components/Kv/GetKey'
import SetKey from '@/components/Kv/SetKey'

export default {
  name: 'KvProofOfConcept',
  components: {
    GetKey,
    SetKey
  },
  props: {
    lucidKeyProp: {
      type: String,
      required: false,
      default: null
    }
  },
  data() {
    return {
      lucidKey: ''
    }
  },
  computed: {
    ...mapGetters(['isKvLoading'])
  },
  async mounted() {
    // Check if a key was passed as a route parameter
    if (this.lucidKeyProp) {
      this.lucidKey = this.lucidKeyProp
      // Load the passed key
      // Wait for next Vue tick as lucid key was not
      // propagated to the component yet
      return this.$nextTick().then(() => this.loadSelectedKey())
    }
  },
  methods: {
    loadSelectedKey() {
      if (this.lucidKey) return this.$refs.getKeyComponent.refreshValue()
    }
  }
}
</script>
