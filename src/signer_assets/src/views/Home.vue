<template>
  <div class="home">

    <el-row class="row-btn" justify="center">
      <el-button class="login-btn" type="primary" :disabled="param.logedIn" @click="login">LOGIN</el-button>
    </el-row>

    <el-row class="row-bg" justify="center">
      <el-input
        class="principal-id"
        v-model="param.principalId"
        :rows="2"
        type="textarea"
        disabled
        placeholder="My Principal ID:"
      />
    </el-row>

    <el-row class="row-btn" justify="center">
      <el-button class="manage-api-key" type="primary" :disabled="!param.logedIn" @click="manageApiKey"
        >GENERATE API KEY</el-button
      >
    </el-row>

    <el-row class="row-btn" justify="center">
      <el-button class="gen-private-key" type="primary" :disabled="!param.logedIn" @click="genPrivkey"
        >GENERATE PRIVATE KEY</el-button
      >
    </el-row>

    <el-row class="row-bg" justify="center">
      <el-input
        class="digest"
        v-model="param.digest"
        :rows="3"
        type="textarea"
        :disabled="!param.logedIn"
        placeholder="Digest to Sign: (Hex String)"
      />
    </el-row>

    <el-row class="row-btn" justify="start" :gutter="24">
      <el-col :offset="5" :span="7"
        ><el-button
          class="ic-sign-btn"
          type="primary"
          :disabled="!param.logedIn || param.signing"
          @click="signByIC"
          >IC ECDSA SIGN</el-button
        ></el-col
      >
      <el-col :offset="0" :span="7"
        ><el-button
          class="sign-btn"
          type="primary"
          :disabled="!param.logedIn || param.signing"
          @click="openSignBox"
          >ECDSA SIGN</el-button
        ></el-col
      >
    </el-row>

    <el-row class="row-bg" justify="center">
      <el-input
        class="result"
        v-model="param.result"
        readonly
        :rows="10"
        type="textarea"
        placeholder="Results to show"
      />
    </el-row>
  </div>
</template>

<script>
import {
  ElSelect,
  ElOption,
  ElInput,
  ElRow,
  ElCol,
  ElButton,
  ElMessageBox,
  ElMessage,
} from "element-plus";
import "element-plus/es/components/message/style/css";
import {
  handleAuthenticated,
  getBackendActor,
} from "../lib";
import { AuthClient } from "@dfinity/auth-client";

const days = BigInt(1);
const hours = BigInt(24);
const nanoseconds = BigInt(3600000000000);

let authClient = undefined;

export default {
  name: "IC Signer",
  props: {
    // msg: String
  },
  data() {
    return {
      param: {
        principal: undefined,
        logedIn: false,
        digest: "",
        actor: undefined,
        principalId: "",
        signing: false,
        result: "",
      },
    };
  },
  components: {
    ElSelect,
    ElOption,
    ElInput,
    ElRow,
    ElCol,
    ElButton,
    ElMessageBox,
    ElMessage,
  },
  methods: {
    async initAuth() {
      authClient = await AuthClient.create();
      if (await authClient.isAuthenticated()) {
        handleAuthenticated(authClient);
      }
    },
    handleLoginSuccess(identity) {
      this.param.logedIn = true;
      this.param.principal = identity;

      this.param.actor = getBackendActor(identity);
      const pricipal = identity.getPrincipal().toString();

      // 这里显示自己的 Principal ID
      this.param.principalId = `My Principal ID:
${pricipal.toString()}`;
    },
    async login() {
      // await this.handleLoginSuccess("anonymous"); // just for test
      await authClient.login({
        onSuccess: async () => {
          handleAuthenticated(authClient);
          this.handleLoginSuccess(authClient.getIdentity());
        },
        identityProvider:
          process.env.NODE_ENV === "production" ? 
            "https://identity.ic0.app/#authorize" :
            process.env.LOCAL_II_CANISTER,
        // Maximum authorization expiration is 8 days
        maxTimeToLive: days * hours * nanoseconds,
      });
    },
    async manageApiKey() {
      this.setResultText("Generating API Key ...", true);
      try {
        let res = await this.param.actor.generate_apikey();
        this.setResultText(`API key generated: ${res}`, true);
      } catch (err) {
        const error = "Failed to generate a API key: \n" + err;
        this.setResultText(error, true);
      }
    },
    async genPrivkey() {
      this.setResultText("Generating Private Key ...", true);
      try {
        let res = await this.param.actor.generate_privkey();
        let genRes = {};
        if (res[1] !== "") {
          genRes.keyId = res[0];
          genRes.publickey = res[1];
          this.setResultText(genRes);
        } else {
          const error = "Failed to generate a private key: \n" + res[0];
          this.setResultText(error, true);
        }
      } catch (err) {
        const error = "Failed to generate a private key: \n" + err;
        this.setResultText(error, true);
      }
    },
    checkDigest() {
      return /^[0-9a-fA-F]{64}$/.test(this.param.digest);
    },
    async openSignBox() {
      if (!this.checkDigest(this.param.digest)) {
        this.setResultText("Invalid Digest Input", true);
        return;
      }

      let result;
      const prompt = `Enter the Key ID`
      try {
        result = await ElMessageBox.prompt(prompt, "Key ID", {
          confirmButtonText: "Confirm",
          cancelButtonText: "Cancel",
          // customClass: "key-id-box",
          // inputPattern:
          //   /(ht|f)tp(s?)\:\/\/[0-9a-zA-Z]([-.\w]*[0-9a-zA-Z])*(:(0-9)*)*(\/?)([a-zA-Z0-9\-\.\?\,\'\/\\\+&amp;%$#_]*)?/,
          inputErrorMessage: "Invalid Key ID",
        });
      } catch (error) {
        // ElMessage({
        //   type: "info",
        //   message: "Send canceled",
        // });
      }
      if (!result) {
        return;
      }
      this.setResultText("Signing ...", true);

      await this.sign(result.value);
    },
    async sign(keyId) {
      if (!this.checkDigest(this.param.digest)) {
        this.setResultText("Invalid Digest Input", true);
        return;
      }

      this.setResultText("Signing ...", true);
      this.param.signing = true;
      try {
        let res = await this.param.actor.sign_digest_mpc(this.param.digest, keyId);
      
        let sig = JSON.parse(res);
        sig.digest = Buffer.from(sig.digest).toString("hex");
        sig.signature = Buffer.from(sig.signature).toString("hex");
        sig.publickey = Buffer.from(sig.publickey).toString("hex");
        this.setResultText(sig);
      } catch (err) {
        const error = "Failed to sign: \n" + err;
        this.setResultText(error, true);
      }
      this.param.signing = false;
    },
    async signByIC() {
      this.setResultText("Signing By IC ...", true);
      this.param.signing = true;
      try {
        let res = await this.param.actor.sign_digest_ic(this.param.digest);
        this.setResultText(res);
      } catch (err) {
        const error = "Failed to call sign_digest_ic: \n" + err;
        this.setResultText(error, true);
      }
      this.param.signing = false;
    },
    setResultText(message, text) {
      this.param.result = text ? message : JSON.stringify(message, null, 2);
    },
  },
  async mounted() {
    await this.initAuth();
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
.row-bg {
  padding: 10px 10px;
  background-color: #f9fafc;
}

.principal-id {
  width: 58.32%;
}

.row-btn {
  padding: 10px 0px;
  background-color: #f9fafc;
}

.login-btn {
  width: 57.64%;
}

.manage-api-key {
  width: 57.64%;
}

.gen-private-key {
  width: 57.64%;
}

.digest {
  width: 58.32%;
}

.view-proposals-btn {
  width: 57.64%;
}

.ic-sign-btn {
  width: 100%;
}

.sign-btn {
  width: 100%;
}

.result {
  width: 58.32%;
}
</style>
<style>
.node-box {
  width: 40%;
}
</style>
