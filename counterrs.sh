# run from nix/src

ambs --no-color --column -r throw+[a-zA-Z]*Error > ../../rawerrors.txt
ambs --no-color --column "SQLiteError(" >> ../../rawerrors.txt
ambs --no-color --column "SQLiteBusy(" >> ../../rawerrors.txt
ambs --no-color --column "NotDeterministic(" >> ../../rawerrors.txt
ambs --no-color --column "NoSuchBinaryCacheFile(" >> ../../rawerrors.txt
ambs --no-color --column "SubstError(" >> ../../rawerrors.txt
ambs --no-color --column "BuildError(" >> ../../rawerrors.txt
ambs --no-color --column "InvalidPath(" >> ../../rawerrors.txt
ambs --no-color --column "Unsupported(" >> ../../rawerrors.txt
ambs --no-color --column "SubstituteGone(" >> ../../rawerrors.txt
ambs --no-color --column "SubstituterDisabled(" >> ../../rawerrors.txt
ambs --no-color --column "NotInStore(" >> ../../rawerrors.txt
ambs --no-color --column "PathInUse(" >> ../../rawerrors.txt
ambs --no-color --column "UploadToHTTP(" >> ../../rawerrors.txt
ambs --no-color --column "BadStorePath(" >> ../../rawerrors.txt
ambs --no-color --column "JSONParseError(" >> ../../rawerrors.txt
ambs --no-color --column "EvalError(" >> ../../rawerrors.txt
ambs --no-color --column "ParseError(" >> ../../rawerrors.txt
ambs --no-color --column "AssertionError(" >> ../../rawerrors.txt
ambs --no-color --column "ThrownError(" >> ../../rawerrors.txt
ambs --no-color --column "Abort(" >> ../../rawerrors.txt
ambs --no-color --column "TypeError(" >> ../../rawerrors.txt
ambs --no-color --column "UndefinedVarError(" >> ../../rawerrors.txt
ambs --no-color --column "RestrictedPathError(" >> ../../rawerrors.txt
ambs --no-color --column "AttrPathNotFound(" >> ../../rawerrors.txt
ambs --no-color --column "NoPositionInfo(" >> ../../rawerrors.txt
ambs --no-color --column "UnknownCompressionMethod(" >> ../../rawerrors.txt
ambs --no-color --column "CompressionError(" >> ../../rawerrors.txt
ambs --no-color --column "BadHash(" >> ../../rawerrors.txt
ambs --no-color --column "BadURL(" >> ../../rawerrors.txt
ambs --no-color --column "UsageError(" >> ../../rawerrors.txt
ambs --no-color --column "Error(" >> ../../rawerrors.txt
ambs --no-color --column "SerialisationError(" >> ../../rawerrors.txt
ambs --no-color --column "EndOfFile(" >> ../../rawerrors.txt
ambs --no-color --column "Interrupted(" >> ../../rawerrors.txt
ambs --no-color --column "FormatError(" >> ../../rawerrors.txt
ambs --no-color --column "ThreadPoolShutDown(" >> ../../rawerrors.txt
ambs --no-color --column "logWarning(" >> ../../rawerrors.txt
ambs --no-color --column "logError(" >> ../../rawerrors.txt

sed "s/.(B.\[m//g" ../../rawerrors.txt | grep "printError" -v | grep "MakeError" -v | grep "LocalNoInlineNoReturn" -v | sort -u > ../../errors.txt

rm ../../rawerrors.txt 
