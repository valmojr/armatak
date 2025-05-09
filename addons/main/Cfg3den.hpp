class Cfg3den {
  class Object {
    class AttributeCategories {
      class armatak_3den_attributes {
        displayName = "ARMA Team Awareness Kit";
        class Attributes {
          class armatak_attribute_unit_callsign {
            displayName = "Callsign";
            tooltip = "Unit callsign on TAK";
            property = "armatak_attribute_unit_callsign";
            control = "Edit";
            value = "";
            expression = "_this setVariable ['%s',_value];";
            wikiType="[[String]]";
            unique = 0;
            condition = "object";
            typeName = "STRING";
          };
          class armatak_attribute_unit_role {
            displayName = "Role";
            tooltip = "Unit role on TAK";
            property = "armatak_attribute_unit_role";
            control = "Combo";
            class Values {
              class armatak_attribute_unit_role_value_teammember {
                name = "Team Member";
                value = "Team Member";
                default = 1;
              };
              class armatak_attribute_unit_role_value_teamleader {
                name = "Team Leader";
                value = "Team Lead";
              };
              class armatak_attribute_unit_role_value_hq {
                name = "Headquarters";
                value = "HQ";
              };
              class armatak_attribute_unit_role_value_sniper {
                name = "Sniper";
                value = "Sniper";
              };
              class armatak_attribute_unit_role_value_medic {
                name = "Medic";
                value = "Medic";
              };
              class armatak_attribute_unit_role_value_forward_observer {
                name = "Forward Observer";
                value = "Forward Observer";
              };
              class armatak_attribute_unit_role_value_rto {
                name = "Radio Telephone Operator";
                value = "RTO";
              };
              class armatak_attribute_unit_role_value_k9 {
                name = "K9 Operator";
                value = "K9";
              };
            };
            expression = "_this setVariable ['%s',_value];";
            unique = 0;
            condition = "objectBrain";
          };
        };
      };
    };
  };
};