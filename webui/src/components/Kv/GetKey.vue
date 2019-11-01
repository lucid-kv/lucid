<template>
  <div>
    <b-card>
      <Promised :promise="valuePromise" v-slot:combined="{ isPending, data, error }">
        <b-button
          @click="refreshValue"
          variant="primary"
          class="mb-3"
          :disabled="isKvLoading || isPending || lucidKey === ''"
        >
          Load
        </b-button>

        <Loader v-if="isPending" />
        <textarea v-else-if="data" v-text="data"></textarea>
        <b-alert v-else-if="error" show variant="danger" v-text="error" />
      </Promised>
    </b-card>
  </div>
</template>

<script>
import { mapMutations, mapGetters } from 'vuex'

import Loader from '@/components/Loader'
import { Lucid } from '@/lucidApi'

export default {
  components: {
    Loader
  },
  props: {
    lucidKey: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      valuePromise: Promise.resolve()
    }
  },
  computed: {
    ...mapGetters(['isKvLoading'])
  },
  methods: {
    ...mapMutations(['setKvLoading']),

    async refreshValue() {
      this.setKvLoading(true)
      try {
        this.valuePromise = Lucid.getKey(this.lucidKey).then(res => res.json())
        await this.valuePromise
      }
      catch (error) {
        this.valuePromise = Promise.reject(error.message)
      }
      finally {
        this.setKvLoading(false)
      }
    }
  }
}
</script>

<style scoped>
textarea {
  width: 100%;
  max-height: 350px;
  height: 100vh;
}
</style>
