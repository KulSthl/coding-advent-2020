use strict;
use warnings;
    
my $filename = "./data.txt";
open(my $fh, '<:encoding(UTF-8)', $filename)
    or die "Could not open file '$filename' $!";
my @dictionary = ();
for (my $row_count = 0 ; $row_count <= 106; $row_count++) {
    for (my $column_count = 0 ; $column_count <= 7 ; $column_count++) {
        push @dictionary, ($column_count + $row_count * 8);
    }
}

my $result_one = 0;
my $lowest_id = 100000;
while (my $row = <$fh>) {
    chomp $row;
    my $lower_row = 0;
    my $higher_row = 127;
    my $lower_column = 0;
    my $higher_column = 7;
    my $res_row = 0;
    foreach my $char (split //, $row) {
        if($higher_row - $lower_row eq 1){
            if($char eq "F"){
                print "R: $lower_row\n";
                $res_row = $lower_row;
            }
            elsif($char eq "B"){
                print "R: $higher_row\n";
                $res_row = $higher_row;
            }
            else{
                my @array = get_column($char, $lower_column, $higher_column);
                $lower_column = $array[0];
                $higher_column = $array[1];
                if($higher_column == $lower_column){
                    print "C: $lower_column\n";
                    my $id = $lower_column + $res_row * 8;
                    print "Result: $id ";
                    my $size = @dictionary;
                    for (my $dic_i = 0 ; $dic_i < $size ; $dic_i++) {
                        if ($id == $dictionary[$dic_i]){
                            $dictionary[$dic_i] = 0;
                        }
                    }
                    if($result_one < $id){
                        $result_one = $id;
                    }
                    if($lowest_id > $id){
                        $lowest_id = $id;
                    }
                }
            }
        }
        else{
        my @array = get_row($char, $lower_row, $higher_row);
        $lower_row = $array[0];
        $higher_row = $array[1];
        }
    }
    print "$row\n";
}

sub get_row  
{ 
    my $letter = $_[0]; 
    my $lower_row = $_[1];
    my $higher_row = $_[2];
    if($letter eq "F"){
        my $half = ($higher_row-$lower_row) / 2;
        $higher_row = $higher_row - $half;
        $higher_row = int($higher_row)
    }
    if($letter eq "B"){
        my $half = ($higher_row-$lower_row) / 2;
        $lower_row = $lower_row + $half;
        $lower_row = int($lower_row + 0.5)
    }
    print("R: $letter $lower_row - $higher_row \n");
    return($lower_row, $higher_row);
}
sub get_column  
{ 
    my $letter = $_[0]; 
    my $lower_column = $_[1];
    my $higher_column = $_[2];
    if($letter eq "L"){
        my $half = ($higher_column-$lower_column) / 2;
        $higher_column = $higher_column - $half;
        $higher_column = int($higher_column);
    }
    if($letter eq "R"){
        my $half = ($higher_column-$lower_column) / 2;
        $lower_column = $lower_column + $half;
        $lower_column = int($lower_column + 0.5);
    }
    print("C: $letter $lower_column - $higher_column \n");
    return($lower_column, $higher_column);
}
print "highest id and part 1 $result_one\n";
print "lowest id: $lowest_id\n";
foreach my $id (@dictionary) {
    if($result_one > $id && $lowest_id < $id){
        if ($id == 0){
        # print "null";
        }
        else{
            print "My seat $id\n";
        }
    }
}