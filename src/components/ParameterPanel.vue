<template>
    <div>
        <v-form
                ref="form"
                v-model="valid">
            <div v-for="(helix, index) in helixFamily" :key="index">
                <div class="helix-number" @click="setInFocus(index + 1)">
                    <div>helix {{index+1}}</div>
                    <div v-if="index + 1 !==inputInFocus">
                        <v-icon> mdi-chevron-down </v-icon>
                    </div>
                    <div v-if="index + 1 ===inputInFocus">
                        <v-icon> mdi-chevron-up</v-icon>
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
    </div>
</template>

<script>
    export default {
        name: "ParameterPanel",
        data: () => ({
            valid: false,
            numberRules: [v => !!v || 'parameter is required',],
            helixFamily: [{'radius':'', 'rise': '', 'frequency': '', 'unit_size': ''}],
            inputInFocus: 1
        }),
        methods: {
            computeHelix() {
                // first we make the string input to numeric
                let helix;
                for (helix of this.helixFamily) {
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
                this.helixFamily.push( {'radius':'', 'rise': '', 'frequency': '', 'unit_size': ''} )
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

                for ( let [key, value] of searchParams ) {
                    key = key.replace( '#','' );  // remove the # from the parameter (only needed for the first key)

                    if ( key === 'radius' ) {  // we assume that the previous helix was specified if we add new radius
                        newHelixFamily.push( {} );  // add a new helix object
                        helix = newHelixFamily[newHelixFamily.length -1];  // get the last helix object from family
                    }
                    helix[key] =  value
                }

                if (newHelixFamily.length > 0) {
                    // we are done parsing, let's send it over for calculation
                    console.log(`query string loaded for: ${newHelixFamily.length} helix objects`);
                    this.helixFamily = newHelixFamily;
                    this.computeHelix()
                } else {
                    console.log('No query string parsed.')
                }
            }
        },
        mounted() {
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

    .fade-enter-active {
        transition: opacity 0.2s;
    }
    .fade-enter /* .fade-leave-active below version 2.1.8 */ {
        opacity: 0;
    }
</style>
