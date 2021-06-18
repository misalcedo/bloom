task :compile do
    system('cargo build', exception: true)

    Dir.chdir(File.join(__dir__, 'target/debug')) do
        lib_dir = RbConfig::CONFIG['libdir']
        ld_shared = RbConfig::CONFIG['LDSHARED']
        out_flag = RbConfig::CONFIG['OUTFLAG']
        ld_flags = RbConfig::CONFIG['LDFLAGS']
        dld_flags = RbConfig::CONFIG['DLDFLAGS']
        args_shared = RbConfig::CONFIG['LIBRUBYARG_SHARED']
        dl_ext = RbConfig::CONFIG['DLEXT']

        command = "#{ld_shared} #{out_flag}bloom.#{dl_ext} libbloom.dylib #{ld_flags}#{dld_flags} #{args_shared}"

        print "Executing command: #{command}"

        system(command, exception: true)
    end


end

task :console do
    require "#{__dir__}/target/debug/bloom"
    require "irb"

    ARGV.clear()
    IRB.start()
end

task :default => :compile
