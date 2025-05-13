class Cfg3den {
  class Object {
    class AttributeCategories {
      class armatak_3den_attributes {
        displayName = "ARMA Team Awareness Kit";
        class Attributes {
          class armatak_attribute_unit_callsign {
            displayName = "Unit Callsign";
            tooltip = "Unit callsign on TAK";
            property = "armatak_attribute_unit_callsign";
            control = "Edit";
            expression = "_this setVariable ['armatak_attribute_unit_callsign',_value]";
            defaultValue = "''";
            condition = "objectBrain";
            typeName = "STRING";
          };
          class armatak_attribute_unit_role {
            displayName = "Unit Role";
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
          class armatak_attribute_marker_callsign {
            displayName = "Marker Callsign";
            tooltip = "Marker Callsign on TAK";
            property = "armatak_attribute_marker_callsign";
            control = "Edit";
            expression = "_this setVariable ['armatak_attribute_marker_callsign',_value]";
            defaultValue = "''";
            condition = "objectVehicle";
            typeName = "STRING";
          };
          class armatak_attribute_marker_type {
            displayName = "Marker Type";
            tooltip = "Marker type on TAK";
            property = "armatak_attribute_marker_type";
            control = "Edit";
            expression = "_this setVariable ['armatak_attribute_marker_type',_value]";
            defaultValue = "''";
            condition = "objectVehicle";
            typeName = "STRING";
          };
        };
      };
    };
  };
  class Group {
    class AttributeCategories {
      class armatak_3den_group_attributes {
        displayName = "ARMA Team Awareness Kit";
        class Attributes {
          class armatak_attribute_group_color {
            displayName = "Color";
            tooltip = "Group on TAK";
            property = "armatak_attribute_group_color";
            control = "Combo";
            class Values {
              class armatak_attribute_group_color_value_white {
                name = "White";
                value = "White";
              };
              class armatak_attribute_group_color_value_yellow {
                name = "Yellow";
                value = "Yellow";
              };
              class armatak_attribute_group_color_value_orange {
                name = "Orange";
                value = "Orange";
              };
              class armatak_attribute_group_color_value_magenta {
                name = "Magenta";
                value = "Magenta";
              };
              class armatak_attribute_group_color_value_red {
                name = "Red";
                value = "Red";
              };
              class armatak_attribute_group_color_value_maroon {
                name = "Maroon";
                value = "Maroon";
              };
              class armatak_attribute_group_color_value_purple {
                name = "Purple";
                value = "Purple";
              };
              class armatak_attribute_group_color_value_darkblue {
                name = "Dark Blue";
                value = "DarkBlue";
              };
              class armatak_attribute_group_color_value_blue {
                name = "Blue";
                value = "Blue";
              };
              class armatak_attribute_group_color_value_cyan {
                name = "Cyan";
                value = "Cyan";
              };
              class armatak_attribute_group_color_value_teal {
                name = "Teal";
                value = "Teal";
              };
              class armatak_attribute_group_color_value_green {
                name = "Green";
                value = "Green";
              };
              class armatak_attribute_group_color_value_darkgreen {
                name = "Dark Green";
                value = "DarkGreen";
              };
              class armatak_attribute_group_color_value_brown {
                name = "Brown";
                value = "Brown";
              };
            };
            expression = "_this setVariable ['%s',_value];";
            unique = 0;
          };
        };
      };
    };
  };
};
