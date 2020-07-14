# run from nix/src

ambs --no-color --column -r throw+[a-zA-Z]*Error > ../../meh.txt
ambs --no-color --column "SQLiteError(" >> ../../meh.txt
ambs --no-color --column "SQLiteBusy(" >> ../../meh.txt
ambs --no-color --column "NotDeterministic(" >> ../../meh.txt
ambs --no-color --column "NoSuchBinaryCacheFile(" >> ../../meh.txt
ambs --no-color --column "SubstError(" >> ../../meh.txt
ambs --no-color --column "BuildError(" >> ../../meh.txt
ambs --no-color --column "InvalidPath(" >> ../../meh.txt
ambs --no-color --column "Unsupported(" >> ../../meh.txt
ambs --no-color --column "SubstituteGone(" >> ../../meh.txt
ambs --no-color --column "SubstituterDisabled(" >> ../../meh.txt
ambs --no-color --column "NotInStore(" >> ../../meh.txt
ambs --no-color --column "PathInUse(" >> ../../meh.txt
ambs --no-color --column "UploadToHTTP(" >> ../../meh.txt
ambs --no-color --column "BadStorePath(" >> ../../meh.txt
ambs --no-color --column "JSONParseError(" >> ../../meh.txt
ambs --no-color --column "EvalError(" >> ../../meh.txt
ambs --no-color --column "ParseError(" >> ../../meh.txt
ambs --no-color --column "AssertionError(" >> ../../meh.txt
ambs --no-color --column "ThrownError(" >> ../../meh.txt
ambs --no-color --column "Abort(" >> ../../meh.txt
ambs --no-color --column "TypeError(" >> ../../meh.txt
ambs --no-color --column "UndefinedVarError(" >> ../../meh.txt
ambs --no-color --column "RestrictedPathError(" >> ../../meh.txt
ambs --no-color --column "AttrPathNotFound(" >> ../../meh.txt
ambs --no-color --column "NoPositionInfo(" >> ../../meh.txt
ambs --no-color --column "UnknownCompressionMethod(" >> ../../meh.txt
ambs --no-color --column "CompressionError(" >> ../../meh.txt
ambs --no-color --column "BadHash(" >> ../../meh.txt
ambs --no-color --column "BadURL(" >> ../../meh.txt
ambs --no-color --column "UsageError(" >> ../../meh.txt
ambs --no-color --column "Error(" >> ../../meh.txt
ambs --no-color --column "SerialisationError(" >> ../../meh.txt
ambs --no-color --column "EndOfFile(" >> ../../meh.txt
ambs --no-color --column "Interrupted(" >> ../../meh.txt
ambs --no-color --column "FormatError(" >> ../../meh.txt
ambs --no-color --column "ThreadPoolShutDown(" >> ../../meh.txt
ambs --no-color --column "logWarning(" >> ../../meh.txt
ambs --no-color --column "logError(" >> ../../meh.txt

sed "s/.(B.\[m//g" ../../meh.txt > ../../wot.txt
cat ../../wot.txt | grep "printError" -v | grep "MakeError" -v | grep "LocalNoInlineNoReturn" -v > ../../wot2.txt
sort -u ../../wot2.txt > ../../exceptions2.txt

rm ../../meh.txt ../../wot.txt
