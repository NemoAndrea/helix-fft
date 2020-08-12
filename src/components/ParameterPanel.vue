<template>
    <div>
        <v-form
                ref="form"
                v-model="valid">
            <div v-for="(helix, index) in helixFamily" :key="index">
                <div class="helix-header">
                    <div class="helix-number" @click="setInFocus(index + 1)">
                        <div>helix {{index+1}}</div>
                        <div class="helix-button-box">
                            <div v-if="index + 1 !==inputInFocus">
                                <v-icon> mdi-chevron-down </v-icon>
                            </div>
                            <div v-if="index + 1 ===inputInFocus">
                                <v-icon> mdi-chevron-up</v-icon>
                            </div>
                        </div>
                    </div>
                    <div v-if="index + 1 ===inputInFocus" class="helix-button-box-focus">
                        <div class="handedness-button">
                            <v-tooltip bottom v-if="helix['handedness'] === 'right'">
                                <template v-slot:activator="{ on }">
                                    <v-icon v-on="on" @click="toggleHandedness(helix)"> mdi-axis-z-rotate-counterclockwise</v-icon>
                                </template>
                                <span>Make left-handed helix</span>
                            </v-tooltip>
                            <v-tooltip bottom v-if="helix['handedness'] === 'left'">
                                <template v-slot:activator="{ on }">
                                    <v-icon v-on="on" @click="toggleHandedness(helix)"> mdi-axis-z-rotate-clockwise</v-icon>
                                </template>
                                <span>Make right handed helix</span>
                            </v-tooltip>
                        </div>
                        <div class="copy-button">
                            <v-tooltip bottom>
                                <template v-slot:activator="{ on }">
                                    <v-icon v-on="on" @click="copyHelixParams(index)"> mdi-content-copy</v-icon>
                                </template>
                                <span>Copy params to new helix</span>
                            </v-tooltip>
                        </div>

                    </div>
                </div>
                <transition name="fade">
                    <div v-show="index+1 === inputInFocus">
                        <v-text-field
                                v-model="helix['radius']"
                                :rules="numberRules"
                                label="Radius"
                                required
                                outlined
                                color="var(--primary)"
                                suffix="nm"
                                type="number"
                        ></v-text-field>

                        <v-text-field
                                v-model="helix['rise']"
                                :rules="numberRules"
                                label="Rise"
                                required
                                outlined
                                color="var(--primary)"
                                suffix="nm"
                                type="number"
                        ></v-text-field>

                        <v-text-field
                                v-model="helix['frequency']"
                                :rules="numberRules"
                                label="Subunits per pitch"
                                required
                                outlined
                                color="var(--primary)"
                                suffix="nm"
                                type="number"
                        ></v-text-field>

                        <v-text-field
                                v-model="helix['unit_size']"
                                :rules="numberRules"
                                label="Unit size"
                                required
                                outlined
                                color="var(--primary)"
                                suffix="nm"
                                type="number"
                        ></v-text-field>

                        <div v-show="index+1 > 1" class="optional-params">
                            <v-text-field
                                    v-model="helix['offset']"
                                    :rules="numberAndZeroRules"
                                    label="Helix Z offset"
                                    outlined
                                    required
                                    color="var(--primary)"
                                    suffix="nm"
                                    type="number"
                            ></v-text-field>
                        </div>
                    </div>
                </transition>
            </div>
            <div class="button-box">
                <v-tooltip bottom>
                    <template v-slot:activator="{ on }">
                        <v-btn
                                dark
                                fab
                                color="var(--primary)"
                                @click="newHelix"
                                v-on="on"
                        >
                            <v-icon>mdi-plus</v-icon>
                        </v-btn>
                    </template>
                    <span>Add another helix</span>
                </v-tooltip>
                <v-tooltip bottom>
                    <template v-slot:activator="{ on }">
                        <v-btn
                                :disabled="!valid"
                                color="var(--primary)"
                                @click="computeHelix"
                                v-on="on">
                            Compute
                        </v-btn>
                    </template>
                    <span>Show helix and compute FFT</span>
                </v-tooltip>
            </div>
        </v-form>
        <v-snackbar
                v-model="snackbar"
        >
            {{ snackText }}
            <template v-slot:action="{ attrs }">
                <v-btn
                        color="pink"
                        text
                        v-bind="attrs"
                        @click="snackbar = false"
                >
                    Close
                </v-btn>
            </template>
        </v-snackbar>
    </div>
</template>

<script>
    export default {
        name: "ParameterPanel",
        data: () => ({
            valid: false,
            numberRules: [
                v => !!v || 'parameter is required',
                v => v > 0 || 'value must be larger than 0',
            ],
            numberAndZeroRules: [
                v => !!v || 'parameter is required',
                v => v >= 0 || 'value must be larger or equal to 0',
            ],
            default_params: {'radius':'', 'rise': '', 'frequency': '', 'unit_size': '', 'offset': '0', 'handedness': 'right'},
            helixFamily: [],
            inputInFocus: 1,
            snackbar: false,
            snackText: 'unspecified',
            modelName: ''
        }),
        methods: {
            computeHelix() {
                // first we make the string input to numeric
                for (let helix of this.helixFamily) {
                    helix['radius'] = Number(helix['radius']);
                    helix['rise'] =  Number(helix['rise']);
                    helix['frequency'] =  Number(helix['frequency']);
                    helix['unit_size'] =  Number(helix['unit_size']);
                }
                // then we send it off to the rest of the application
                this.sendHelixFamily()
            },

            sendHelixFamily() {
                console.log('Sending new helix-family off for computation');
                this.$emit( 'updateHelixFamily', [...this.helixFamily] )
            },

            newHelix() {
                // add a new helix to the family (will expose new form options)
                this.helixFamily.push( this.default_params )
            },

            setInFocus( helix_number ) {
                if (this.inputInFocus  === helix_number) {
                    this.inputInFocus = -1;  // set to nonexistent helix (i.e. collapse all)
                } else {
                    this.inputInFocus = helix_number;
                }
            },

            parseQueryString() {  // this uses not very robust methods
                console.log( 'Checking query string' );
                const searchParams = new URLSearchParams( window.location.hash );

                let newHelixFamily =[];
                let helix = {};
                const validKeys = Object.keys(this.default_params);
                let badKeys = [];

                for ( let [key, value] of searchParams ) {
                    key = key.replace( '#','' );  // remove the # from the parameter (only needed for the first key)

                    if ( key === 'radius' ) {  // we assume that the previous helix was specified if we add new radius
                        newHelixFamily.push( {...this.default_params});  // add a new helix object
                        helix = newHelixFamily[newHelixFamily.length -1];  // get the last helix object from family
                    }

                    // add value to helix if it is a valid key (specified in default_params)
                    if ( validKeys.includes(key) ){
                        helix[key] = value
                    } else if (key === 'name') {
                        console.log('Baby has a name')
                        this.$emit( 'updateName', value );  // this query model has a name. Update parent.
                    } else {
                        badKeys.push(key)
                    }
                }

                //show bad keys in snackbar
                if ( badKeys.length > 0 ) {
                    this.snackbar = true;
                    this.snackText = 'The following invalid keys were ignored: ' + badKeys.join(', ');
                }

                if (newHelixFamily.length > 0) {
                    // we are done parsing, let's send it over for calculation
                    console.log(`query string loaded for: ${newHelixFamily.length} helix objects`);
                    this.helixFamily = newHelixFamily;
                    this.computeHelix()
                } else {
                    console.log('No query string parsed.')
                }
            },

            copyHelixParams(index){
                console.log(`Copying parameters of helix ${index+1} into a new helix`);
                this.helixFamily.push( {...this.helixFamily[index]} );
                this.snackText = `Copied helix parameters from helix ${index+1}`;
                this.snackbar = true;
            },

            toggleHandedness( helix ) {
                if (helix['handedness'] === 'right') {
                    helix['handedness'] = 'left';
                } else if (helix['handedness'] === 'left') {
                    helix['handedness'] = 'right';
                }
            },

            exportModel(name) {
                console.log('exporting with name ' + name);
                let exportString = window.location.hostname;
                exportString += '#';
                if (name) { exportString += 'name=' + name}
                for (let helix of this.helixFamily) {
                    for ( const [key, value] of Object.entries( helix ) ) {
                        if (exportString.slice(-1)!== '#') {  // prevent problems with first entry
                            exportString += '&' + key + '=' + value
                        } else {
                            exportString += key + '=' + value
                        }
                    }
                }

                // copy to user clipboard
                var dummy = document.createElement("textarea");
                document.body.appendChild(dummy);
                dummy.value = exportString;
                dummy.select();
                document.execCommand("copy");
                document.body.removeChild(dummy);
                console.log(exportString);

                this.snackText = 'copied model URL to clipboard';
                this.snackbar = true;
            }
        },
        mounted() {
            this.helixFamily.push(this.default_params);
            this.parseQueryString();

        }
    }
</script>

<style scoped>
    .button-box{
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .helix-number{
        color: var(--primary);
        font-weight: bold;
        margin-bottom: 0.3rem;
        display: flex;
        justify-content: space-between;
        vertical-align: middle;
    }

    .helix-button-box{
       display: flex;
    }

    v-icon.helix-button-box{
        display: flex;
    }

    .helix-header {
        display: flex;
    }

    .helix-number{
        width: auto;
        flex-grow: 1
    }

    .helix-button-box-focus{
        margin-left: 0.5rem;
        display: flex;
    }

    .copy-button{
        margin-left: 0.5rem;
    }

    .fade-enter-active {
        transition: opacity 0.2s;
    }
    .fade-enter /* .fade-leave-active below version 2.1.8 */ {
        opacity: 0;
    }
</style>